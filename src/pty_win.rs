use portable_pty::{native_pty_system, CommandBuilder, PtySize};

pub struct Pty {
    pub child: Box<dyn portable_pty::Child + std::marker::Send + std::marker::Sync>,
    pub reader: Box<dyn std::io::Read + Send>,
    pub writer: Box<dyn std::io::Write + Send>,
}

// impl Pty {
//     pub fn is_alive(&mut self) -> bool {
//         self.child.try_wait().is_ok()
//     }
// }

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
        writer: pair.master.try_clone_writer()?,
    })
}

// ============================================================================
// use std::ffi::OsString;
// use std::io::{Read, Write};
// use winptyrs::{AgentConfig, MouseMode, PTYArgs, PTY};

// pub fn new(command: &str) -> Result<Ptx<PTY>, anyhow::Error> {
//     let cmd = OsString::from(command);
//     let pty_args = PTYArgs {
//         cols: 80,
//         rows: 25,
//         mouse_mode: MouseMode::WINPTY_MOUSE_MODE_NONE,
//         timeout: 10000,
//         agent_config: AgentConfig::WINPTY_FLAG_COLOR_ESCAPES,
//     };

//     // Initialize a pseudoterminal.
//     let mut pty = PTY::new(&pty_args)?;

//     Ok(Ptx { master: pty })
// }
