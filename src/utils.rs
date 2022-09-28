use colored::Colorize;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io::{self, BufRead, Write};
use std::process::exit;
use std::time::SystemTime;
use uuid::Uuid;

pub static mut VERBOSE: bool = false;
pub static mut QUIET: bool = false;

pub fn new_record_file_name(s: &str) -> Result<String, anyhow::Error> {
    let digest = md5::compute(s);
    let uuid = Uuid::parse_str(&format!("{:x}", digest))?;
    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    Ok(format!("{}-{}.wav", uuid, t.as_secs()))
}

pub fn print_status(str: &str) {
    if unsafe { !QUIET } {
        println!("{} {str}", "[*]".yellow());
    }
}

pub fn print_success(str: &str) {
    if unsafe { !QUIET } {
        println!("{} {str}", "[+]".green());
    }
}

pub fn print_verbose(str: &str) {
    if unsafe { VERBOSE } {
        println!("{} {str}", "[*]".white());
    }
}

pub fn print_error(str: &str) {
    if unsafe { !QUIET } {
        eprintln!("{} {str}", "[-]".red());
    }
}

pub fn print_fatal(str: &str) {
    eprintln!("{} {str}", "[!]".red().bold());
    exit(1);
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
