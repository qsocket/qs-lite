use colored::Colorize;
use std::env;
use std::process::exit;

#[cfg(target_os = "windows")]
const DEFAULT_SHELL: &str = "cmd.exe";

#[cfg(target_os = "android")]
const DEFAULT_SHELL: &str = "sh";

#[cfg(all(not(target_os = "windows"), not(target_os = "android")))]
const DEFAULT_SHELL: &str = "bash -il";

const HELP_PROMPT: &str = "USAGE:
qs-lite [FLAGS] [OPTIONS]

FLAGS:
\t-g, --generate         Verbose output mode
\t-h, --help             Prints help information
\t-C, --no-tls           Disable TLS encryption
\t-q, --quiet            Disable output
\t-v, --verbose          Verbose output mode
\t--pin                  Enable certificate fingerprint verification on TLS connections

OPTIONS:
\t-e, --exec <string>    Program to execute [default: bash -il]
\t-n, --probe <int>      Probe interval for calling QSRN [default: 5]
\t-s, --secret <string>  Secret. (e.g. password)
\t-f, --forward <string> IP:PORT for TCP forwarding.
";
const DEFAULT_E2E_CIPHER: &str = "AES-GCM-SHA-256-E2E";

// #[derive(FromArgs)]
/// Reach new heights.
pub struct Options {
    /// disable output.p
    // #[argh(switch, short = 'q')]
    pub quiet: bool,

    /// verbose output mode.
    // #[argh(switch, short = 'v')]
    pub verbose: bool,

    /// verbose output mode.
    // #[argh(switch, short = 'g')]
    pub generate: bool,

    /// probe interval for calling QSRN.
    // #[argh(option, short = 't', default = 5)]
    pub probe: i32,

    /// disable TLS+E2E encryption.
    // #[argh(switch, short = 'C')]
    pub no_encryption: bool,

    /// enable certificate fingerprint verification on TLS connections.
    // #[argh(switch, name = "pin")]
    pub verify_cert: bool,

    /// secret. (e.g. password).
    // #[argh(option, short = 's')]
    pub secret: String,

    /// Program to execute.
    pub exec: String,

    /// TCP Forwarding address
    pub forward_addr: String,
}

pub fn parse_options() -> Result<Options, anyhow::Error> {
    let mut opts: Options = Options {
        quiet: false,
        verbose: false,
        generate: false,
        probe: 5,
        no_encryption: false,
        verify_cert: false,
        secret: "".to_string(),
        exec: DEFAULT_SHELL.to_string(),
        forward_addr: "".to_string(),
    };

    let mut args: Vec<String> = vec!["qs-lite".to_string()];
    if let Ok(env_var) = env::var("QS_ARGS") {
        for var in env_var.split_whitespace() {
            args.append(vec![var.to_string()].as_mut());
        }
    } else {
        args = env::args().collect();
    }

    for i in 0..args.len() {
        if args[i].eq("-e") || args[i].eq("--exec") && ((i + 1) <= args.len()) {
            opts.exec = args[i + 1].to_string();
        } else if (args[i].eq("-s") || args[i].eq("--secret")) && ((i + 1) <= args.len()) {
            opts.secret = args[i + 1].to_string();
        } else if (args[i].eq("-f") || args[i].eq("--forward")) && ((i + 1) <= args.len()) {
            opts.forward_addr = args[i + 1].to_string();
        } else if args[i].eq("-n") || args[i].eq("--probe") && ((i + 1) <= args.len()) {
            opts.probe = args[i + 1].to_string().parse::<i32>().unwrap();
        } else if args[i].eq("-g") || args[i].eq("--generate") {
            opts.generate = true;
        } else if args[i].eq("-C") || args[i].eq("--nocipher") {
            opts.no_encryption = true;
        } else if args[i].eq("--pin") {
            opts.verify_cert = true;
        } else if args[i].eq("-q") || args[i].eq("--quiet") {
            opts.quiet = true;
        } else if args[i].eq("-v") || args[i].eq("--verbose") {
            opts.verbose = true;
        } else if args[i].eq("-h") {
            println!("{}", HELP_PROMPT);
            exit(0x1);
        }
    }

    Ok(opts)
}

pub fn summarize_options(opts: &Options) {
    if opts.quiet {
        return;
    }

    println!(
        "{} {}",
        "[#]".yellow().bold(),
        ".:: Qsocket Lite ::.".blue().bold()
    );
    println!("{} Secret: {}", " ├──>".yellow(), opts.secret.red());
    println!("{} Cert. Pinning: {}", " ├──>".yellow(), opts.verify_cert);
    println!("{} Probe Interval: {}", " ├──>".yellow(), opts.probe);
    if !opts.forward_addr.is_empty() {
        println!("{} Forward: {}", " ├──>".yellow(), opts.forward_addr);
    }
    if opts.no_encryption {
        println!("{} Encryption: {}", " └──>".yellow(), "DISABLED".red().bold());
    }else{
        println!("{} Encryption: {}", " └──>".yellow(), DEFAULT_E2E_CIPHER);
    }

    println!();
}
