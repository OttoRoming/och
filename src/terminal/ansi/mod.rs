//! ANSI escape sequences module
//!
//! This module provides constants and functions for working with ANSI escape sequences
//! for terminal control, colors, and styling.

/// Control Sequence Introducer - equivalent to ESC[
pub const CSI: &str = "\x1B[";

#[cfg(test)]
mod tests;

// Module declarations
pub mod bg;
pub mod color256;
pub mod control;
pub mod cursor;
pub mod erase;
pub mod fg;
pub mod screen;
pub mod style;
pub mod truecolor;
pub mod util;
