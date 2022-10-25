//#![windows_subsystem = "windows"]

// use anyhow::Result;
// use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use rand::Rng;
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
            Ok(_) => (),
            Err(_) => continue,
        }
        qsock.set_write_timeout(Some(Duration::from_millis(100)))?;
        qsock.set_read_timeout(Some(Duration::from_millis(100)))?;
        // Init PTY shell
        let mut proc = pty::new(opts.exec.as_str())?;
        let qsock = Arc::new(Mutex::new(qsock));
        let stream1 = qsock.clone();
        let stream2 = qsock.clone();
        let mut reader = proc.pair.master.try_clone_reader()?;
        let mut writer = proc.pair.master.try_clone_writer()?;

        let t1 = thread::spawn(move || loop {
            let mut rng = rand::thread_rng();
            std::thread::sleep(Duration::from_millis(rng.gen_range(0..10))); // Required for preventing mutex dead lock!
            let mut buf = vec![0; 1024];
            let n = reader.read(&mut buf).unwrap_or(0);
            if n != 0 {
                println!("--> {} bytes", n);
                stream1
                    .lock()
                    .unwrap()
                    .write_all(buf[0..n].as_mut())
                    .unwrap_or_default();
            }
        });

        let t2 = thread::spawn(move || loop {
            let mut rng = rand::thread_rng();
            #[cfg(not(target_os = "windows"))]
            std::thread::sleep(Duration::from_millis(rng.gen_range(0..10))); // Required for preventing mutex dead lock!
            #[cfg(target_os = "windows")]
            std::thread::sleep(Duration::from_millis(rng.gen_range(500..600)));
            let mut buf = vec![0; 1024];
            let n = stream2.lock().unwrap().read(&mut buf).unwrap_or(0);
            if n != 0 {
                println!("<-- {} bytes", n);
                writer.write_all(buf[0..n].as_ref()).unwrap_or_default();
            }
        });

        proc.child.wait()?;
        drop(t1);
        drop(t2);
    }
}
