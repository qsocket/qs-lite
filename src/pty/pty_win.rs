use conpty::io::{PipeReader, PipeWriter};
use conpty::Process;
use std::io::{stdin, stdout, Stdin, Stdout};
use std::process::Command;

pub struct Pty {
    #[allow(dead_code)]
    pub child: Process,
    pub reader: PipeReader,
    pub writer: PipeWriter,
}

pub fn new(command: &str) -> Result<Pty, std::io::Error> {
    let cmd = Command::new(command);
    let mut child = Process::spawn(cmd)?;
    let mut reader = child.output()?;
    let writer = child.input()?;
    reader.blocking(false);

    Ok(Pty {
        child,
        reader,
        writer,
    })
}

pub struct Tty {
    pub reader: Stdin,
    pub writer: Stdout,
}

pub fn get_current_tty() -> Result<Tty, std::io::Error> {
    Ok(Tty {
        reader: stdin(),
        writer: stdout(),
    })
}
