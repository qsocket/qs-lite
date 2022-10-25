use colored::Colorize;
use std::env;
use std::process::exit;

#[cfg(target_os = "windows")]
const DEFAULT_SHELL: &str = "cmd.exe";

#[cfg(not(target_os = "windows"))]
const DEFAULT_SHELL: &str = "bash -il";

const HELP_PROMPT: &str = "USAGE:
qs-lite [FLAGS] [OPTIONS]

FLAGS:
\t-g, --generate         Verbose output mode
\t-h, --help             Prints help information
\t-n, --no-tls           Disable TLS encryption
\t-q, --quiet            Disable output
\t-v, --verbose          Verbose output mode
\t--pin                  Enable certificate fingerprint verification on TLS connections

OPTIONS:
\t-e, --exec <string>    Program to execute [default: bash -il]
\t-p, --probe <int>      Probe interval for calling QSRN [default: 5]
\t-s, --secret <string>  Secret. (e.g. password)
";

// #[derive(StructOpt, Debug)]
// #[structopt(name = "basic")]
pub struct Options {
    /// Disable output.
    // #[structopt(short, long)]
    pub quiet: bool,

    /// Verbose output mode.
    // #[structopt(short, long)]
    pub verbose: bool,

    /// Verbose output mode.
    // #[structopt(short, long)]
    pub generate: bool,

    /// Probe interval for calling QSRN.
    // #[structopt(short, long, default_value = "5")]
    pub probe: i32,

    /// Disable TLS encryption.
    // #[structopt(short, long)]
    pub no_tls: bool,

    /// Enable certificate fingerprint verification on TLS connections.
    // #[structopt(short, long)]
    pub verify_cert: bool,

    /// Secret. (e.g. password).
    // #[structopt(short, long)]
    pub secret: String,

    /// Program to execute.
    // #[structopt(short, long, default_value = DEFAULT_SHELL)]
    pub exec: String,
}

pub fn parse_options() -> Result<Options, anyhow::Error> {
    let mut opts: Options = Options {
        quiet: false,
        verbose: false,
        generate: false,
        probe: 5,
        no_tls: false,
        verify_cert: false,
        secret: "".to_string(),
        exec: DEFAULT_SHELL.to_string(),
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
        } else if args[i].eq("-n") || args[i].eq("--probe") && ((i + 1) <= args.len()) {
            opts.probe = args[i + 1].to_string().parse::<i32>().unwrap();
        } else if args[i].eq("-g") || args[i].eq("--generate") {
            opts.generate = true;
        } else if args[i].eq("-C") || args[i].eq("--no-tls") {
            opts.no_tls = true;
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
    println!("{} Secret: {}", "├──>".yellow(), opts.secret.red());
    println!("{} TLS: {}", "├──>".yellow(), !opts.no_tls);
    println!("{} Cert. Pinning: {}", "├──>".yellow(), opts.verify_cert);
    println!("{} Probe Interval: {}", "└──>".yellow(), opts.probe);
}
