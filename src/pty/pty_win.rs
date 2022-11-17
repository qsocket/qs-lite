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

// pub struct Pty {
//     pub child: conpty::Process,
//     pub reader: conpty::io::PipeReader,
//     pub writer: conpty::io::PipeWriter,
// }

// pub fn new(command: &str) -> Result<Pty, anyhow::Error> {
//     let proc = conpty::spawn(command)?;

//     let ptyin = proc.input()?;
//     let ptyout = proc.output()?;

//     Ok(Pty {
//         child: proc,
//         reader: ptyout,
//         writer: ptyin,
//     })
// }
