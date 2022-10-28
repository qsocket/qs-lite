// use nix::pty;
// use std::fs::File;
// use std::os::unix::io::FromRawFd;
// use std::process::{Child, Command, Stdio};

// pub struct Pty {
//     pub child: Child,
//     pub master: File,
//     pub slave: File,
// }

// pub fn new(command: &str) -> Result<Pty, anyhow::Error> {
//     let ws = pty::Winsize {
//         ws_row: 30,
//         ws_col: 120,
//         ws_xpixel: 0,
//         ws_ypixel: 0,
//     };

//     // If there is a window for the process, then I need to pass in window size here
//     let fds = nix::pty::openpty(Some(&ws), None)?;

//     let master = fds.master;
//     let slave = fds.slave;

//     let parts = command.split_whitespace().collect::<Vec<&str>>();
//     let mut builder = Command::new(parts[0]);
//     builder.stdin(unsafe { Stdio::from_raw_fd(slave) });
//     builder.stdout(unsafe { Stdio::from_raw_fd(slave) });
//     builder.stderr(unsafe { Stdio::from_raw_fd(slave) });
//     if command.contains(char::is_whitespace) {
//         builder.args(&parts[1..]);
//     }

//     let process = builder.spawn()?;
//     let pty = Pty {
//         child: process,
//         master: unsafe { File::from_raw_fd(master) },
//         slave: unsafe { File::from_raw_fd(slave) },
//     };

//     Ok(pty)
// }

// ==============================================================================================================

use portable_pty::{native_pty_system, CommandBuilder, PtySize};
pub struct Pty {
    pub child: Box<dyn portable_pty::Child + std::marker::Send + std::marker::Sync>,
    pub pair: portable_pty::PtyPair,
}

pub fn new(command: &str) -> Result<Pty, anyhow::Error> {
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
    if command.contains(char::is_whitespace) {
        builder.args(&parts[1..]);
    }
    let child = pair.slave.spawn_command(builder)?;
    Ok(Pty { child, pair })
}
