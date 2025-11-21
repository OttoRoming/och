use super::ansi::*;

pub fn info(message: &str) {
    println!(
        "{cyan}{bold}->{reset} {message}",
        cyan = fg::CYAN,
        bold = style::BOLD,
        reset = style::RESET,
        message = message
    );
}

pub fn warn(message: &str) {
    println!(
        "{yellow}{bold}-->{reset} {message}",
        yellow = fg::YELLOW,
        bold = style::BOLD,
        reset = style::RESET,
        message = message
    );
}

pub fn err(message: &str) {
    println!(
        "{red}{bold}==>{reset} {message}",
        red = fg::RED,
        bold = style::BOLD,
        reset = style::RESET,
        message = message
    );
}
