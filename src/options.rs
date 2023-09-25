use clap::Parser;
use colored::Colorize;
use std::net::SocketAddr;
use thiserror::Error;

#[cfg(target_os = "windows")]
const DEFAULT_SHELL: &str = "cmd.exe";

#[cfg(target_os = "android")]
const DEFAULT_SHELL: &str = "sh";

#[cfg(all(not(target_os = "windows"), not(target_os = "android")))]
const DEFAULT_SHELL: &str = "bash -il";

const DEFAULT_E2E_CIPHER: &str = "AES-GCM-SHA-256-E2E (Prime: 4096)";

#[derive(Error, Debug)]
pub enum ArgParseError {
    #[error("Invalid forward address")]
    ForwardAddress,
    #[error("Invalid proxy address")]
    ProxyAddress,
    #[error("Invalid TLS certificate fingerprint")]
    CertificateFingerprint,
}

/// QSocket toolkit options.
#[derive(Parser, Debug)]
#[command(name = "QSocket Lite")]
#[command(version = "1.0.2")]
#[command(about = "QSocket Toolkit.", long_about = None)]
pub struct Options {
    /// secret. (e.g. password).
    #[arg(long, short = 's', default_value_t = String::new())]
    pub secret: String,

    /// program to execute.
    #[arg(long, short = 'e', default_value_t = DEFAULT_SHELL.to_string())]
    pub exec: String,

    /// forward address (IP:PORT) for traffic forwarding.
    #[arg(long, short = 'f', default_value_t = String::new())]
    pub forward_addr: String,

    /// user socks proxy address for connecting QSRN.
    #[arg(long, short = 'x', default_value_t = String::new())]
    pub proxy_addr: String,

    /// hex encoded TLS certificate fingerprint for validation.
    #[arg(long, short = 'X', default_value_t = String::new())]
    pub cert_fingerprint: String,

    /// probe interval for connecting QSRN.
    #[arg(long, short = 'n', default_value_t = 5)]
    pub probe: u64,

    /// disable all (TLS+E2E) encryption.
    #[arg(long, short = 'C')]
    pub no_encryption: bool,

    /// disable End-to-End encryption.
    #[arg(long)]
    pub no_e2e: bool,

    /// initiate a full PTY (interactive) shell.
    #[arg(long, short)]
    pub interactive: bool,

    /// server mode. (listen for connections)
    #[arg(long, short = 'l')]
    pub listen: bool,

    /// generate a random secret.
    #[arg(long, short = 'g')]
    pub generate: bool,

    /// use TOR network for connecting QSRN.
    #[arg(long, short = 'T')]
    pub use_tor: bool,

    /// generate a QR code with given stdin and print on the terminal.
    #[arg(long)]
    pub qr: bool,

    /// quiet mode. (no stdout)
    #[arg(long, short = 'q')]
    pub quiet: bool,

    /// verbose output mode.
    #[arg(long, short = 'v')]
    pub verbose: bool,
}

pub fn parse_options() -> Result<Options, ArgParseError> {
    // let mut opts: Options = argh::from_env();
    let mut opts = Options::parse();

    if opts.use_tor {
        opts.proxy_addr = String::from("127.0.0.1:9050");
    }
    // Check if the proxy address if valid
    if !opts.proxy_addr.is_empty() && opts.proxy_addr.parse::<SocketAddr>().is_err() {
        return Err(ArgParseError::ProxyAddress);
    }
    // Check if the forward address if valid
    if !opts.forward_addr.is_empty() && opts.forward_addr.parse::<SocketAddr>().is_err() {
        let parts: Vec<&str> = opts.forward_addr.split(':').collect();
        if parts.len() != 3
            || parts[0].to_string().parse::<i32>().is_err()
            || format!("{}:{}", parts[1], parts[2])
                .parse::<SocketAddr>()
                .is_err()
        {
            return Err(ArgParseError::ForwardAddress);
        }
    }
    // Check if the certificate fingerprint is valid
    if !opts.cert_fingerprint.is_empty() && hex::decode(&opts.cert_fingerprint).is_err() {
        return Err(ArgParseError::CertificateFingerprint);
    }

    Ok(opts)
}

pub fn summarize_options(opts: &Options) {
    if opts.quiet {
        return;
    }
    let mut mode = String::from("client");
    let mut enc_mode = DEFAULT_E2E_CIPHER.to_string();

    if opts.listen {
        mode = String::from("server");
    }
    if opts.no_encryption {
        enc_mode = "DISABLED".red().bold().to_string();
    } else if opts.no_e2e {
        enc_mode = String::from("TLS");
    }

    println!(
        "{} {}",
        "[#]".yellow().bold(),
        ".:: QSocket Lite ::.".blue().bold()
    );
    println!("{} Secret: {}", " ├──>".yellow(), opts.secret.red());
    println!("{} Mode: {}", " ├──>".yellow(), mode);
    if !opts.cert_fingerprint.is_empty() {
        println!("{} Cert. Pinning: true", " ├──>".yellow());
    }
    println!("{} Probe Interval: {}", " ├──>".yellow(), opts.probe);
    if !opts.proxy_addr.is_empty() {
        println!("{} Proxy: {}", " ├──>".yellow(), opts.proxy_addr);
    }
    if !opts.forward_addr.is_empty() {
        println!("{} Forward: {}", " ├──>".yellow(), opts.forward_addr);
    }
    println!("{} Encryption: {}", " └──>".yellow(), enc_mode);
    println!();
}
