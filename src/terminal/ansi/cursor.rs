/// Move cursor to home position (0, 0)
pub const HOME: &str = "\x1B[H";

/// Move cursor up by `n` lines
pub fn up(n: u16) -> String {
    format!("{}[{}A", super::control::ESC, n)
}

/// Move cursor down by `n` lines
pub fn down(n: u16) -> String {
    format!("{}[{}B", super::control::ESC, n)
}

/// Move cursor right by `n` columns
pub fn forward(n: u16) -> String {
    format!("{}[{}C", super::control::ESC, n)
}

/// Move cursor left by `n` columns
pub fn backward(n: u16) -> String {
    format!("{}[{}D", super::control::ESC, n)
}

/// Move cursor to beginning of next line, `n` lines down
pub fn next_line(n: u16) -> String {
    format!("{}[{}E", super::control::ESC, n)
}

/// Move cursor to beginning of previous line, `n` lines up
pub fn previous_line(n: u16) -> String {
    format!("{}[{}F", super::control::ESC, n)
}

/// Move cursor to column `n`
pub fn horizontal_absolute(n: u16) -> String {
    format!("{}[{}G", super::control::ESC, n)
}

/// Move cursor to specific position (line, column)
pub fn position(line: u16, column: u16) -> String {
    format!("{}[{};{}H", super::control::ESC, line, column)
}

/// Alternative form for cursor position
pub fn position_alt(line: u16, column: u16) -> String {
    format!("{}[{};{}f", super::control::ESC, line, column)
}

/// Request cursor position (reports as ESC[#;#R)
pub const GET_POSITION: &str = "\x1B[6n";

/// Move cursor one line up, scrolling if needed
pub const MOVE_UP_SCROLL: &str = "\x1BM";

/// Save cursor position (DEC)
pub const SAVE_DEC: &str = "\x1B7";

/// Restore cursor position (DEC)
pub const RESTORE_DEC: &str = "\x1B8";

/// Save cursor position (SCO)
pub const SAVE_SCO: &str = "\x1B[s";

/// Restore cursor position (SCO)
pub const RESTORE_SCO: &str = "\x1B[u";
