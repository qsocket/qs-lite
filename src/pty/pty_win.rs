use conpty::io::{PipeReader, PipeWriter};
use conpty::Process;
use std::fs::File;
use std::os::windows::prelude::FromRawHandle;
use std::os::windows::prelude::RawHandle;
use std::process::Command;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};

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
    pub console: conpty::console::Console,
    pub reader: File,
    pub writer: File,
}

pub fn handle_to_rawhandle(h: &HANDLE) -> RawHandle {
    RawHandle::from(h.0 as *mut std::ffi::c_void)
}

pub fn get_current_tty() -> Result<Tty, std::io::Error> {
    let stdout_handle = match unsafe { GetStdHandle(STD_OUTPUT_HANDLE) } {
        Err(_) => return Err(std::io::ErrorKind::Other.into()),
        Ok(h) => h,
    };

    let stdin_handle = match unsafe { GetStdHandle(STD_INPUT_HANDLE) } {
        Err(_) => return Err(std::io::ErrorKind::Other.into()),
        Ok(h) => h,
    };

    let h_out = std::os::windows::prelude::RawHandle::from(handle_to_rawhandle(&stdout_handle));
    let stdout = unsafe { File::from_raw_handle(h_out) };

    let h_in = std::os::windows::prelude::RawHandle::from(handle_to_rawhandle(&stdin_handle));
    let stdin = unsafe { File::from_raw_handle(h_in) };

    Ok(Tty {
        console: conpty::console::Console::current()?,
        reader: stdin,
        writer: stdout,
    })
}
