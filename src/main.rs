// #![windows_subsystem = "windows"]
use anyhow::anyhow;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use log::{error, info, warn, LevelFilter};
use qsocket::{QSocketError, SocketType};
use spinoff::*;
use std::io::ErrorKind;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod options;
mod pty;
mod utils;

const TIMEOUT: u64 = 30;
static LOGGER: utils::Logger = utils::Logger;

fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);
    let mut opts = match options::parse_options() {
        Ok(o) => o,
        Err(e) => {
            error!("{e}");
            return;
        }
    };

    if opts.quiet {
        log::set_max_level(LevelFilter::Off);
    }
    if opts.verbose {
        log::set_max_level(LevelFilter::Trace);
    }

    if opts.qr {
        if let Err(e) = utils::generate_qr() {
            error!("{e}");
        }
        return;
    }

    if !opts.listen && opts.generate {
        print!("{}", utils::random_secret());
        return;
    }
    if opts.generate {
        opts.secret = utils::random_secret();
    }
    if opts.secret.is_empty() {
        // Ask for a secret if none
        opts.secret = utils::read_line("Enter Secret (or press Enter to generate): ");
        if opts.secret.is_empty() {
            opts.secret = utils::random_secret();
        }
    }

    // Print option summary
    options::summarize_options(&opts);
    if opts.listen {
        info!("Listening for connections...");
        loop {
            if let Err(e) = probe_qsrn(&opts) {
                match e {
                    QSocketError::KnockFail => (),
                    QSocketError::KnockBusy => {
                        warn!("Another server already listening with the same secret!");
                        error!("Exiting...");
                        return;
                    }
                    _ => error!("{e}"),
                }
            }
            thread::sleep(Duration::from_secs(opts.probe));
        }
    }

    if let Err(e) = connect(&opts) {
        error!("{e}");
    }
}

fn connect(opts: &options::Options) -> Result<(), anyhow::Error> {
    let mut qsock = qsocket::QSocket::new(qsocket::PeerType::Client, &opts.secret);
    let mut spnr = Spinner::new(spinner!(["|", "/", "-", "\\"], 50), " ...", None);
    let mut local_stream: Option<TcpStream> = None;
    if !opts.forward_addr.is_empty() {
        let fw_parts: Vec<&str> = opts.forward_addr.split(':').collect();
        if fw_parts.len() == 3 {
            qsock.set_forward_addr(format!("{}:{}", fw_parts[1], fw_parts[2]))?;
            spnr.update_text(" Waiting for local connection...");
            let listener = TcpListener::bind(format!("0.0.0.0:{}", fw_parts[0]))?;
            let (stream, addr) = listener.accept()?;
            stream.set_nonblocking(true)?;
            spnr.update_text(format!(" Got new connection from {}", addr));
            local_stream = Some(stream);
        } else {
            qsock.set_forward_addr(opts.forward_addr.clone())?;
        }
    }

    if !opts.proxy_addr.is_empty() {
        todo!("Implement proxy...");
    }

    spnr.update_text(" Dialing qsocket relay network...");
    let mut dial_type: SocketType = SocketType::E2E;
    if opts.no_encryption {
        dial_type = SocketType::TCP;
    } else if opts.no_e2e {
        dial_type = SocketType::TLS;
    }

    if let Err(e) = qsock.dial_with(dial_type) {
        spnr.stop_and_persist("", "");
        return Err(anyhow!(e));
    }
    qsock.set_nonblocking(true)?;
    if local_stream.is_some() {
        spnr.update_text(" Forwarding local traffic...");
        let receiver = utils::bind_stream(qsock, local_stream.unwrap())?;
        let e = receiver.recv()?;
        spnr.stop_and_persist("", "");
        return Err(anyhow!(e));
    } else {
        spnr.stop_and_persist("", "");
        attach(qsock, opts.interactive)?;
    }
    Ok(())
}

fn attach(qsock: qsocket::QSocket, interactive: bool) -> Result<(), anyhow::Error> {
    if interactive {
        enable_raw_mode()?;
    }

    utils::wait_for_sigint(3);
    let tty = pty::get_current_tty()?;
    let reader = Arc::new(Mutex::new(tty.reader));
    let writer = Arc::new(Mutex::new(tty.writer));

    let qsock = Arc::new(Mutex::new(qsock));
    let (sender, receiver) = std::sync::mpsc::channel();

    thread::spawn(move || loop {
        if let Err(e) = utils::copy_until(qsock.clone(), writer.clone(), TIMEOUT) {
            if e.kind() == ErrorKind::BrokenPipe || e.kind() == ErrorKind::ConnectionAborted {
                let _ = sender.send(true);
                break;
            }
        }
        if let Err(e) = utils::copy_until(reader.clone(), qsock.clone(), TIMEOUT) {
            if e.kind() == ErrorKind::BrokenPipe || e.kind() == ErrorKind::ConnectionAborted {
                let _ = sender.send(true);
                break;
            }
        }
    });

    receiver.recv()?;
    warn!("Connection closed.");
    if interactive {
        disable_raw_mode()?;
    }
    Ok(())
}

fn probe_qsrn(opts: &options::Options) -> Result<(), QSocketError> {
    let mut qsock = qsocket::QSocket::new(qsocket::PeerType::Server, &opts.secret);
    if !opts.proxy_addr.is_empty() {
        todo!("Implement proxy...");
    }

    let mut dial_type: SocketType = SocketType::E2E;
    if opts.no_encryption {
        dial_type = SocketType::TCP;
    } else if opts.no_e2e {
        dial_type = SocketType::TLS;
    }
    qsock.dial_with(dial_type)?;
    qsock.set_nonblocking(true)?;
    info!("Starting new session...");
    // utils::wait_for_sigint(3);
    // Check if a forward address is given
    if qsock.get_forward_addr().is_some() {
        return forward_traffic(qsock);
    }

    // Init PTY shell
    let pty = pty::new(opts.exec.as_str())?;
    let reader = Arc::new(Mutex::new(pty.reader));
    let writer = Arc::new(Mutex::new(pty.writer));
    let qsock = Arc::new(Mutex::new(qsock));
    let mut exit = false;

    thread::spawn(move || loop {
        if exit {
            let _ = qsock.lock().unwrap().shutdown(std::net::Shutdown::Both);
            info!("Session closed.");
            break;
        }
        if let Err(e) = utils::copy_until(reader.clone(), qsock.clone(), TIMEOUT) {
            if e.kind() == ErrorKind::BrokenPipe || e.kind() == ErrorKind::ConnectionAborted {
                exit = true;
                continue;
            }
        }
        if let Err(e) = utils::copy_until(qsock.clone(), writer.clone(), TIMEOUT) {
            if e.kind() == ErrorKind::BrokenPipe || e.kind() == ErrorKind::ConnectionAborted {
                exit = true;
                continue;
            }
        }
    });

    Ok(())
}

fn forward_traffic(qsock: qsocket::QSocket) -> Result<(), QSocketError> {
    info!(
        "Forwarding traffic to {}...",
        qsock.get_forward_addr().unwrap()
    );
    let sock = std::net::TcpStream::connect(qsock.get_forward_addr().unwrap())?;
    sock.set_nonblocking(true)?;
    let receiver = utils::bind_stream(qsock, sock)?;
    Err(qsocket::QSocketError::IoError(receiver.recv()?))
}
