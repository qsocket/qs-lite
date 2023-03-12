//#![windows_subsystem = "windows"]

use anyhow::anyhow;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use std::time::Duration;

mod options;
mod pty;
mod utils;

const TIMEOUT: u64 = 50;
const FORWARD_BUF_SIZE: usize = 4096;

fn main() -> Result<(), anyhow::Error> {
    let mut opts = options::parse_options()?;
    // unsafe { utils::QUIET = opts.quiet };

    if opts.generate {
        opts.secret = utils::random_secret();
    }

    if opts.secret.is_empty() {
        // Ask for secret.
        opts.secret = utils::read_line("Enter Secret (or press Enter to generate): ");
        if opts.secret.is_empty() {
            opts.secret = utils::random_secret();
        }
    }
    // print option summary
    options::summarize_options(&opts);

    loop {
        match probe_qsrn(&opts) {
            Ok(_) => (),
            Err(e) => {
                if e.to_string() != qsocket::ERR_KNOCK_FAILED {
                    utils::print_error(&e.to_string(), opts.quiet);
                }
            }
        }
        std::thread::sleep(time::Duration::from_secs(opts.probe as u64));
    }
} // the stream is closed here

fn forward_traffic(
    qsock: &mut qsocket::QSocket,
    opts: &options::Options,
) -> Result<(), anyhow::Error> {
    let mut sock = std::net::TcpStream::connect(opts.forward_addr.as_str())?;
    sock.set_nonblocking(true)?;

    loop {
        let mut buf = vec![0; FORWARD_BUF_SIZE];
        match qsock.read(&mut buf) {
            Ok(n) => {
                if let Err(e) = sock.write_all(&buf[0..n]) {
                    if !e.to_string().contains("os error 11") {
                        utils::print_error(e.to_string().as_str(), opts.quiet);
                        break;
                    }
                }
            }
            Err(e) => {
                if !e.to_string().contains("os error 11") {
                    utils::print_error(e.to_string().as_str(), opts.quiet);
                    break;
                }
            }
        }

        match sock.read(&mut buf) {
            Ok(n) => {
                if let Err(e) = qsock.write_all(&buf[0..n]) {
                    if !e.to_string().contains("os error 11") {
                        utils::print_error(e.to_string().as_str(), opts.quiet);
                        break;
                    }
                }
            }
            Err(e) => {
                if !e.to_string().contains("os error 11") {
                    utils::print_error(e.to_string().as_str(), opts.quiet);
                    break;
                }
            }
        }
    }
    Ok(())
}

fn probe_qsrn(opts: &options::Options) -> Result<(), anyhow::Error> {
    let mut qsock = qsocket::QSocket::new(&opts.secret, opts.verify_cert);
    qsock.add_id_tag(qsocket::peer_id_tag::SERVER)?;
    if !opts.forward_addr.is_empty() {
        qsock.add_id_tag(qsocket::peer_id_tag::PROXY)?;
    }

    if opts.no_tls {
        qsock.dial_tcp()?;
    } else {
        qsock.dial()?;
    }
    // Check if a forward address is given
    if !opts.forward_addr.is_empty() {
        qsock.set_nonblocking(true)?;
        return forward_traffic(&mut qsock, opts);
    }

    // Configure qsock settings
    #[cfg(target_os = "windows")]
    qsock.set_nonblocking(true)?;
    qsock.set_write_timeout(Some(Duration::from_millis(TIMEOUT)))?;
    qsock.set_read_timeout(Some(Duration::from_millis(TIMEOUT)))?;

    // Init PTY shell
    #[cfg(target_os = "windows")]
    let mut pty = pty::new(opts.exec.as_str())?;
    #[cfg(not(target_os = "windows"))]
    let pty = pty::new(opts.exec.as_str())?;
    let reader = Arc::new(Mutex::new(pty.reader));
    let writer = Arc::new(Mutex::new(pty.writer));
    let qsock = Arc::new(Mutex::new(qsock));
    let (sender, receiver) = std::sync::mpsc::channel();

    thread::spawn(move || loop {
        if receiver.try_recv().is_ok() {
            let _ = qsock.lock().unwrap().shutdown(std::net::Shutdown::Both);
            break;
        }
        copy_until(reader.clone(), qsock.clone(), TIMEOUT).unwrap_or_default();
        copy_until(qsock.clone(), writer.clone(), TIMEOUT).unwrap_or_default();
    });

    pty.child.wait();
    let _ = sender.send(true);
    Ok(())
}

pub fn copy_until<S, D>(
    reader: Arc<Mutex<S>>,
    writer: Arc<Mutex<D>>,
    dur: u64,
) -> Result<usize, anyhow::Error>
where
    S: Read + std::marker::Send + 'static,
    D: Write + std::marker::Send + 'static,
{
    let (sender, receiver) = std::sync::mpsc::channel();
    let t = thread::spawn(move || {
        let mut buf = vec![0; 4096];
        let n = reader.lock().unwrap().read(&mut buf).unwrap_or(0);
        if n > 0 {
            writer
                .lock()
                .unwrap()
                .write_all(&buf[0..n])
                .unwrap_or_default();
        }
        sender.send(n)
    });
    drop(t);
    match receiver.recv_timeout(Duration::from_millis(dur)) {
        Ok(n) => Ok(n),
        Err(e) => Err(anyhow!(e)),
    }
}
