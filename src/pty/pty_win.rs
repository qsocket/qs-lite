use portable_pty::{native_pty_system, CommandBuilder, PtySize};

pub struct Pty {
    pub child: Box<dyn portable_pty::Child + std::marker::Send + std::marker::Sync>,
    pub reader: Box<dyn std::io::Read + Send>,
    pub writer: Box<dyn portable_pty::MasterPty + Send>,
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

    Ok(Pty {
        child,
        reader: pair.master.try_clone_reader()?,
        writer: pair.master,
    })
}

// ============================================================================
// use std::ffi::OsString;
// use std::io::{Read, Write};
// use winptyrs::{AgentConfig, MouseMode, PTYArgs, PTY};

// impl Write for Pty {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         match &self.writer.write(buf) {
//             Ok(n) => Ok(n),
//             Err(e) => Err(anyhow::anyhow!(e)),
//         }
//     }

//     fn flush(&mut self) -> std::io::Result<()> {
//         Ok(())
//     }
// }
// impl Read for Pty {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         match &self.writer.read(buf.len(), false) {
//             Ok(foo) => buf = foo,
//             Err(e) => Err(anyhow::anyhow!(e)),
//         }
//     }
// }

// pub struct Pty {
//     pub child: PTY,
//     pub reader: PTY,
//     pub writer: PTY,
// }

// pub fn new(command: &str) -> Result<Pty, anyhow::Error> {
//     let cmd = OsString::from(command);
//     let pty_args = PTYArgs {
//         cols: 80,
//         rows: 25,
//         mouse_mode: MouseMode::WINPTY_MOUSE_MODE_NONE,
//         timeout: 10000,
//         agent_config: AgentConfig::WINPTY_FLAG_COLOR_ESCAPES,
//     };

//     // Initialize a pseudoterminal.
//     let mut pty = PTY::new(&pty_args).unwrap();

//     Ok(Pty {
//         child: pty,
//         reader: pty,
//         writer: pty,
//     })
// }
