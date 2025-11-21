/// Set foreground color using 256-color palette
pub fn foreground(color_id: u8) -> String {
    format!("{}[38;5;{}m", super::control::ESC, color_id)
}

/// Set background color using 256-color palette
pub fn background(color_id: u8) -> String {
    format!("{}[48;5;{}m", super::control::ESC, color_id)
}
