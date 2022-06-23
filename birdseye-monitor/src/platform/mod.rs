// Use windows specific implemetaions if building for windows
#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;

// Use linux specific implemetaions if building for linux
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;
