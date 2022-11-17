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
    start_probing_qsrn(&opts)?;

    Ok(())
} // the stream is closed here

fn start_probing_qsrn(opts: &options::Options) -> Result<(), anyhow::Error> {
    let mut first_run = true;
    loop {
        if !first_run {
            std::thread::sleep(time::Duration::from_secs(opts.probe as u64));
        }
        first_run = false;
        let mut qsock = qsocket_rs::Qsocket::new(&opts.secret, qsocket_rs::TAG_ID_NC);
        match qsock.dial(!opts.no_tls, opts.verify_cert) {
            std::result::Result::Ok(_) => (),
            Err(e) => {
                utils::print_error(&e.to_string(), opts.quiet);
                continue;
            }
        }
        #[cfg(target_os = "windows")]
        qsock.set_nonblocking(true)?;
        qsock.set_write_timeout(Some(Duration::from_millis(TIMEOUT)))?;
        qsock.set_read_timeout(Some(Duration::from_millis(TIMEOUT)))?;
        // Init PTY shell
        let pty = pty::new(opts.exec.as_str())?;
        // let reader = master.try_clone()?; // master.try_clone_reader()?;
        // let writer = master.try_clone()?; // .try_clone_writer()?;
        let reader = Arc::new(Mutex::new(pty.reader));
        let writer = Arc::new(Mutex::new(pty.writer));
        let qsock = Arc::new(Mutex::new(qsock));

        loop {
            // if !pty.is_alive() {
            //     break;
            // }
            copy_until(reader.clone(), qsock.clone(), TIMEOUT).unwrap_or_default();
            copy_until(qsock.clone(), writer.clone(), TIMEOUT).unwrap_or_default();
        }
    }
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
