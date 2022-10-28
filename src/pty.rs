use nix::pty;
use std::fs::File;
use std::os::unix::io::FromRawFd;
use std::os::unix::process::CommandExt;
use std::process::Command;

// pub struct Pty {
//     pub child: std::process::Child,
//     pub master: File,
//     pub slave: File,
// }

pub fn new(command: &str) -> Result<File, anyhow::Error> {
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

    if pty.fork_result.is_child() {
        // p.pid = std::process::id();
        builder.exec();
        std::process::exit(0);
    }

    if pty.fork_result.is_parent() {
        return Ok(unsafe { File::from_raw_fd(pty.master) });
    }

    Err(anyhow::anyhow!("Unexpected error!"))
}

// ==============================================================================================================

// use portable_pty::{native_pty_system, CommandBuilder, PtySize};
// pub struct Pty {
//     pub child: Box<dyn portable_pty::Child + std::marker::Send + std::marker::Sync>,
//     pub pair: portable_pty::PtyPair,
// }

// pub fn new(command: &str) -> Result<Pty, anyhow::Error> {
//     // Init PTY shell
//     let pty_system = native_pty_system();
//     // Create a new pty
//     let pair = pty_system
//         .openpty(PtySize {
//             rows: 30,
//             cols: 120,
//             pixel_width: 0,
//             pixel_height: 0,
//         })
//         .unwrap();

//     let parts = command.split_whitespace().collect::<Vec<&str>>();
//     let mut builder = CommandBuilder::new(parts[0]);
//     if command.contains(char::is_whitespace) {
//         builder.args(&parts[1..]);
//     }
//     let child = pair.slave.spawn_command(builder)?;
//     Ok(Pty { child, pair })
// }
