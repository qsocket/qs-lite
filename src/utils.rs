use chrono::offset::Local;
use colored::Colorize;
use core::time;
use log::warn;
use log::{Level, Metadata, Record};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io::{self, stdin, BufRead, ErrorKind, Read, Write};
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
static SIGINT_COUNTER: AtomicU8 = AtomicU8::new(0);

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() < Level::Trace
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = Local::now();
            match record.level() {
                Level::Trace => println!(
                    "[{}] {} {}",
                    now.format("%Y-%m-%d %H:%M:%S"),
                    "TRACE".bold(),
                    record.args()
                ),
                Level::Debug => println!(
                    "[{}] {} {}",
                    now.format("%Y-%m-%d %H:%M:%S"),
                    "DEBUG".bold(),
                    record.args()
                ),
                Level::Info => println!(
                    "[{}] {} {}",
                    now.format("%Y-%m-%d %H:%M:%S"),
                    "INFO".blue().bold(),
                    record.args()
                ),
                Level::Warn => println!(
                    "[{}] {} {}",
                    now.format("%Y-%m-%d %H:%M:%S"),
                    "WARN".yellow().bold(),
                    record.args()
                ),
                Level::Error => println!(
                    "[{}] {} {}",
                    now.format("%Y-%m-%d %H:%M:%S"),
                    "ERROR".red().bold(),
                    record.args()
                ),
            }
            // println!("{} - {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}

pub fn read_line(str: &str) -> String {
    print!("{} {str}", "[>]".blue());
    let _ = io::stdout().flush();
    return io::stdin().lock().lines().next().unwrap().unwrap();
}

pub fn random_secret() -> String {
    let s: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();
    s
}

pub fn generate_qr() -> Result<(), anyhow::Error> {
    let mut buf = Vec::new();
    stdin().read_to_end(&mut buf)?;
    let qr_str = qr2term::generate_qr_string(buf)?;
    println!("\n\t{}", str::replace(&qr_str, "\n", "\n\t"));
    Ok(())
}

pub fn copy_until<S, D>(
    reader: Arc<Mutex<S>>,
    writer: Arc<Mutex<D>>,
    dur: u64,
) -> Result<usize, std::io::Error>
where
    S: Read + std::marker::Send + 'static,
    D: Write + std::marker::Send + 'static,
{
    let (sender, receiver) = std::sync::mpsc::channel();
    thread::spawn(move || {
        let mut buf = vec![0; 4096];
        let n = match reader.lock().unwrap().read(&mut buf) {
            Ok(n) => n,
            Err(e) => return sender.send(Err(e)),
        };
        if let Err(e) = writer.lock().unwrap().write_all(&buf[..n]) {
            return sender.send(Err(e));
        }

        sender.send(Ok(n))
    });
    match receiver.recv_timeout(Duration::from_millis(dur)) {
        Ok(r) => r,
        _ => Err(std::io::ErrorKind::TimedOut.into()),
    }
}

pub fn bind_stream<S, D>(p1: S, p2: D) -> Result<Receiver<io::Error>, std::io::Error>
where
    S: Read + Write + std::marker::Send + 'static,
    D: Read + Write + std::marker::Send + 'static,
{
    let p1 = Arc::new(Mutex::new(p1));
    let p2 = Arc::new(Mutex::new(p2));
    let (sender, receiver) = std::sync::mpsc::channel();
    thread::spawn(move || loop {
        if let Err(e) = copy_until(p1.clone(), p2.clone(), 20) {
            if e.kind() == ErrorKind::BrokenPipe || e.kind() == ErrorKind::ConnectionAborted {
                let _ = sender.send(e);
                break;
            }
        }
        if let Err(e) = copy_until(p2.clone(), p1.clone(), 20) {
            if e.kind() == ErrorKind::BrokenPipe || e.kind() == ErrorKind::ConnectionAborted {
                let _ = sender.send(e);
                break;
            }
        }
    });

    Ok(receiver)
}

#[allow(dead_code)]
pub fn wait_for_sigint(limit: u8) {
    let _ = ctrlc::set_handler(move || {
        let mut counter = SIGINT_COUNTER.load(SeqCst);
        if counter == limit {
            warn!("Exiting...");
            std::process::exit(0x00);
        }
        counter += 1;
        SIGINT_COUNTER.store(counter, SeqCst);
    });

    thread::spawn(|| loop {
        SIGINT_COUNTER.store(0, SeqCst);
        thread::sleep(time::Duration::from_secs(2));
    });
}

// #[allow(dead_code)]
// pub fn wait_for_sigint(limit: u8) -> Result<(), anyhow::Error> {
//     let mut signals = Signals::new(&[SIGINT])?;
//     thread::spawn(move || {
//         for _sig in signals.forever() {
//             println!("Got new sig: {_sig}");
//             let mut counter = SIGINT_COUNTER.load(SeqCst);
//             if counter == limit {
//                 warn!("Exiting...");
//                 std::process::exit(0x00);
//             }
//             counter += 1;
//             SIGINT_COUNTER.store(counter, SeqCst);
//         }
//     });
//
//     thread::spawn(|| loop {
//         SIGINT_COUNTER.store(0, SeqCst);
//         thread::sleep(time::Duration::from_secs(2));
//     });
//     Ok(())
// }
