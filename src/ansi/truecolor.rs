/// Set foreground color using RGB values
pub fn foreground(r: u8, g: u8, b: u8) -> String {
    format!("{}[38;2;{};{};{}m", super::control::ESC, r, g, b)
}

/// Set background color using RGB values
pub fn background(r: u8, g: u8, b: u8) -> String {
    format!("{}[48;2;{};{};{}m", super::control::ESC, r, g, b)
}
