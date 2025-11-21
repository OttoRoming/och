/// Enable line wrapping
pub const ENABLE_LINE_WRAP: &str = "\x1B[=7h";
/// Disable line wrapping
pub const DISABLE_LINE_WRAP: &str = "\x1B[=7l";

/// Common private modes
pub mod private {
    /// Make cursor invisible
    pub const HIDE_CURSOR: &str = "\x1B[?25l";
    /// Make cursor visible
    pub const SHOW_CURSOR: &str = "\x1B[?25h";
    /// Save screen
    pub const SAVE_SCREEN: &str = "\x1B[?47h";
    /// Restore screen
    pub const RESTORE_SCREEN: &str = "\x1B[?47l";
    /// Enable alternative buffer
    pub const ENABLE_ALT_BUFFER: &str = "\x1B[?1049h";
    /// Disable alternative buffer
    pub const DISABLE_ALT_BUFFER: &str = "\x1B[?1049l";
}

/// Set screen mode
pub fn set_mode(mode: u16) -> String {
    format!("{}[={}h", super::control::ESC, mode)
}

/// Reset screen mode
pub fn reset_mode(mode: u16) -> String {
    format!("{}[={}l", super::control::ESC, mode)
}
