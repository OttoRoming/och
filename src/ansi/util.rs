use super::{cursor, erase, style};

/// Clear the entire screen and home cursor
pub fn clear_screen() -> String {
    format!("{}{}", erase::SCREEN, cursor::HOME)
}

/// Clear the current line and move cursor to start
pub fn clear_line() -> String {
    format!("{}{}", erase::LINE, super::control::CR)
}

/// Create a styled string with reset
pub fn styled_text(text: &str, styles: &[&str]) -> String {
    let style_codes: String = styles.join("");
    format!("{}{}{}", style_codes, text, style::RESET)
}

/// Create colored text (foreground only)
pub fn colored_text(text: &str, color_code: &str) -> String {
    format!("{}{}{}", color_code, text, style::RESET)
}

/// Create colored text with both foreground and background
pub fn colored_text_bg(text: &str, fg_color: &str, bg_color: &str) -> String {
    format!("{}{}{}{}", fg_color, bg_color, text, style::RESET)
}
