use super::*;

#[test]
fn test_cursor_movement() {
    assert_eq!(cursor::up(5), "\x1B[5A");
    assert_eq!(cursor::position(10, 20), "\x1B[10;20H");
}

#[test]
fn test_colors() {
    assert_eq!(fg::RED, "\x1B[31m");
    assert_eq!(color256::foreground(42), "\x1B[38;5;42m");
    assert_eq!(truecolor::foreground(255, 128, 0), "\x1B[38;2;255;128;0m");
}

#[test]
fn test_styled_text() {
    let styled = util::styled_text("Hello", &[style::BOLD, fg::RED]);
    assert_eq!(styled, "\x1B[1m\x1B[31mHello\x1B[0m");
}
