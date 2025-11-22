use owo_colors::OwoColorize;

pub fn info(message: &str) {
    println!(
        "{arrow} {message}",
        arrow = "->".blue().bold(),
        message = message
    );
}

pub fn warn(message: &str) {
    println!(
        "{arrow} {message}",
        arrow = "-->".yellow().bold(),
        message = message
    );
}

pub fn err(message: &str) {
    println!(
        "{arrow} {message}",
        arrow = "==>".red().bold(),
        message = message
    );
}
