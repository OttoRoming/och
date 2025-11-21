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

// #[macro_export]
// macro_rules! info {
//     ($($arg:tt)*) => {
//         println!("{} {}", console::style("->").cyan().bold(), format_args!($($arg)*));
//     };
// }

// #[macro_export]
// macro_rules! warn {
//     ($($arg:tt)*) => {
//         println!("{} {}", console::style("-->").yellow().bold(), format_args!($($arg)*));
//     };
// }

// #[macro_export]
// macro_rules! err {
//     ($($arg:tt)*) => {
//         println!("{} {}", console::style("==>").red().bold(), format_args!($($arg)*));
//     };
// }
