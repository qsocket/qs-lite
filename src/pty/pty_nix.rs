use nix::pty;
use nix::sys::wait::WaitPidFlag;
use nix::unistd::Pid;
use std::fs::File;
use std::os::unix::io::FromRawFd;
use std::os::unix::process::CommandExt;
use std::process::Command;

pub struct Pty {
    pub child: Child,
    pub reader: std::fs::File,
    pub writer: std::fs::File,
}

pub struct Child {
    pid: u32,
}

impl Child {
    pub fn wait(&self) {
        match nix::sys::wait::waitpid(Pid::from_raw(self.pid as i32), Some(WaitPidFlag::WSTOPPED)) {
            Ok(s) => println!("Wait finished => {:?}", s),
            Err(e) => println!("WaitPID error: {}", e),
        };
    }
}

pub fn new(command: &str) -> Result<Pty, anyhow::Error> {
    let ws = pty::Winsize {
        ws_row: 30,
        ws_col: 120,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let parts = command.split_whitespace().collect::<Vec<&str>>();
    let mut builder = Command::new(parts[0]);
    if command.contains(char::is_whitespace) {
        builder.args(&parts[1..]);
    }
    let pty = unsafe { nix::pty::forkpty(Some(&ws), None)? };
    let (sender, receiver) = std::sync::mpsc::channel();
    if pty.fork_result.is_child() {
        while sender.send(std::process::id()).is_err() {}
        builder.exec();
        std::process::exit(0);
    }

    if pty.fork_result.is_parent() {
        let pid = receiver.recv()?;
        let master = unsafe { File::from_raw_fd(pty.master) };
        return Ok(Pty {
            child: Child { pid },
            reader: master.try_clone()?,
            writer: master.try_clone()?,
        });
    }

    Err(anyhow::anyhow!("PTY creation failed!"))
}
