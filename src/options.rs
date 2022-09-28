use clap::{arg, App, Command};
use colored::Colorize;
use std::env;

pub struct CommandParams {
    pub exec: String,
    pub secret: String,
    pub probe: i32,
    pub generate: bool,
    pub no_tls: bool,
    pub verify_cert: bool,
    pub verbose: bool,
    pub quiet: bool,
}

pub fn parse_options() -> Result<CommandParams, anyhow::Error> {
    let app = App::new("qs-lite")
        .version("1.0")
        .author("Ege BALCI. <egebalci@pm.me>")
        .about("Qsocket lite shell.")
        .arg(arg!(-e --exec [INPUT] "Program to execute.").default_value(get_os_shell()))
        .arg(arg!(-s --secret [INPUT] "Secret. (e.g. password)."))
        .arg(arg!(-g --generate  "Generate a random secret."))
        .arg(arg!(-t --probe [INPUT] "Probe interval for QSRN.").default_value("5"))
        .arg(arg!(-C --notls  "Disable TLS encryption."))
        .arg(arg!(--pin  "Enable certificate fingerprint verification on TLS connections."))
        .arg(arg!(-q --quiet "Disable output."))
        .arg(arg!(-v --verbose "Verbose output."));
    //.arg(arg!(-T --tor "Use TOR."))

    let matches: clap::ArgMatches;
    if let Ok(mut env_args) = env::var("QS_ARGS") {
        let mut args = vec!["qs-lite"];
        args.append(env_args.split_whitespace().collect::<Vec<&str>>().as_mut());
        matches = app.get_matches_from(args);
    } else {
        matches = app.get_matches();
    }

    let empty = &String::new();
    // Create the command parameters struct
    let exec: &String = matches.get_one("exec").unwrap();
    let secret: &String = matches.get_one("secret").unwrap_or(empty);
    let probe: &String = matches.get_one("probe").unwrap();

    let opts = CommandParams {
        exec: exec.to_string(),
        secret: secret.to_string(),
        probe: probe.parse::<i32>().unwrap(),
        generate: matches.is_present("generate"),
        no_tls: matches.is_present("notls"),
        verify_cert: matches.is_present("pin"),
        verbose: matches.is_present("verbose"),
        quiet: matches.is_present("quiet"),
    };

    Ok(opts)
}

pub fn summarize_options(opts: &CommandParams) {
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
    //     print!("{:?}TLS: {:?}\n", "├──> ".yellow(), !opts.no_tls);
    println!("{} Probe Interval: {}", "└──>".yellow(), opts.probe);
}

fn get_os_shell() -> &'static str {
    if cfg!(windows) {
        return "cmd.exe";
    }
    "/bin/bash"
}
