/// Reset all styles and colors
pub const RESET: &str = "\x1B[0m";

/// Bold mode
pub const BOLD: &str = "\x1B[1m";
/// Dim/faint mode
pub const DIM: &str = "\x1B[2m";
/// Italic mode
pub const ITALIC: &str = "\x1B[3m";
/// Underline mode
pub const UNDERLINE: &str = "\x1B[4m";
/// Blinking mode
pub const BLINK: &str = "\x1B[5m";
/// Inverse/reverse mode
pub const REVERSE: &str = "\x1B[7m";
/// Hidden/invisible mode
pub const HIDDEN: &str = "\x1B[8m";
/// Strikethrough mode
pub const STRIKETHROUGH: &str = "\x1B[9m";

/// Reset bold/dim mode
pub const RESET_BOLD_DIM: &str = "\x1B[22m";
/// Reset italic mode
pub const RESET_ITALIC: &str = "\x1B[23m";
/// Reset underline mode
pub const RESET_UNDERLINE: &str = "\x1B[24m";
/// Reset blinking mode
pub const RESET_BLINK: &str = "\x1B[25m";
/// Reset inverse/reverse mode
pub const RESET_REVERSE: &str = "\x1B[27m";
/// Reset hidden mode
pub const RESET_HIDDEN: &str = "\x1B[28m";
/// Reset strikethrough mode
pub const RESET_STRIKETHROUGH: &str = "\x1B[29m";
