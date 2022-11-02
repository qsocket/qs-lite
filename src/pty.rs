#[cfg(not(target_os = "windows"))]
mod pty_nix;
#[cfg(not(target_os = "windows"))]
pub use pty_nix::*;
#[cfg(target_os = "windows")]
mod pty_win;
#[cfg(target_os = "windows")]
pub use pty_win::*;
