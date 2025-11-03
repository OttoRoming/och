pub mod data;
pub mod details;

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{} {}", console::style("->").cyan().bold(), format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("{} {}", console::style("-->").yellow().bold(), format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        println!("{} {}", console::style("==>").red().bold(), format_args!($($arg)*));
    };
}
