/// Erase from cursor to end of screen
pub const SCREEN_TO_END: &str = "\x1B[0J";
/// Erase from cursor to beginning of screen
pub const SCREEN_TO_START: &str = "\x1B[1J";
/// Erase entire screen
pub const SCREEN: &str = "\x1B[2J";
/// Erase saved lines
pub const SAVED_LINES: &str = "\x1B[3J";
/// Erase from cursor to end of line
pub const LINE_TO_END: &str = "\x1B[0K";
/// Erase from start of line to cursor
pub const LINE_TO_START: &str = "\x1B[1K";
/// Erase entire line
pub const LINE: &str = "\x1B[2K";

/// Alias for SCREEN_TO_END
pub const DEFAULT_SCREEN: &str = "\x1B[J";
/// Alias for LINE_TO_END
pub const DEFAULT_LINE: &str = "\x1B[K";
