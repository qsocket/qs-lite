use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::env;

pub struct Pty {
    pub child: PtyChild,
    pub reader: Box<dyn std::io::Read + Send>,
    pub writer: Box<dyn portable_pty::MasterPty + Send>,
}

pub struct PtyChild {
    child: Box<dyn portable_pty::Child + std::marker::Send + std::marker::Sync>,
}

impl PtyChild {
    pub fn wait(&mut self) {
        let _ = &self.child.wait();
    }
}

pub fn new(command: &str) -> Result<Pty, std::io::Error> {
    // Init PTY shell
    let pty_system = native_pty_system();
    // Create a new pty
    let pair = pty_system
        .openpty(PtySize {
            rows: 30,
            cols: 120,
            pixel_width: 0,
            pixel_height: 0,
        })
        .unwrap();

    let parts = command.split_whitespace().collect::<Vec<&str>>();
    let mut builder = CommandBuilder::new(parts[0]);

    builder.env("qs_netcat", env::current_exe()?);
    if command.contains(char::is_whitespace) {
        builder.args(&parts[1..]);
    }
    let child = pair.slave.spawn_command(builder)?;

    Ok(Pty {
        child: PtyChild { child: child },
        reader: pair.master.try_clone_reader()?,
        writer: pair.master,
    })
}
