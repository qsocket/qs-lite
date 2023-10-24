use nix::pty;
use nix::sys::wait::waitpid;
use nix::unistd::Pid;
use std::fs::File;
use std::os::unix::io::FromRawFd;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::{env, io};

pub struct Pty {
    pub child: Child,
    pub reader: std::fs::File,
    pub writer: std::fs::File,
}

pub struct Tty {
    pub reader: std::fs::File,
    pub writer: std::fs::File,
}

pub struct Child {
    #[allow(dead_code)]
    pid: i32,
}

impl Child {
    #[allow(dead_code)]
    pub fn wait(&self) {
        let _ = waitpid(Pid::from_raw(self.pid), None);
    }
}

pub fn new(command: &str) -> Result<Pty, std::io::Error> {
    let ws = pty::Winsize {
        ws_row: 30,
        ws_col: 120,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let parts = command.split_whitespace().collect::<Vec<&str>>();
    let mut builder = Command::new(parts[0]);
    builder.env("qs_netcat", env::current_exe()?);
    if command.contains(char::is_whitespace) {
        builder.args(&parts[1..]);
    }
    let pty = unsafe { nix::pty::forkpty(Some(&ws), None)? };
    if pty.fork_result.is_child() {
        builder.exec();
        std::process::exit(0);
    }

    if pty.fork_result.is_parent() {
        let pid = -1; // All childs
        let master = unsafe { File::from_raw_fd(pty.master) };
        return Ok(Pty {
            child: Child { pid },
            reader: master.try_clone()?,
            writer: master.try_clone()?,
        });
    }

    Err(io::ErrorKind::Other.into())
}

pub fn get_current_tty() -> Result<Tty, std::io::Error> {
    let tty = termion::get_tty()?;
    Ok(Tty {
        reader: tty.try_clone()?,
        writer: tty.try_clone()?,
    })
}
