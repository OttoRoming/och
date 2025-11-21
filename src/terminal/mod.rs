mod ansi;
pub mod log;
mod progress;

use std::io;

fn size() -> io::Result<libc::winsize> {
    let mut window_size: libc::winsize = unsafe { std::mem::zeroed() };
    let result = unsafe {
        libc::ioctl(
            libc::STDOUT_FILENO,
            libc::TIOCGWINSZ.into(),
            &mut window_size,
        )
    };

    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(window_size)
    }
}

pub use progress::Progress;
