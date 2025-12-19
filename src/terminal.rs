use std::env;

use crate::color::ColorMode;

/// Detect terminal color capability.
pub fn detect_color_mode() -> ColorMode {
    if env::var("NO_COLOR").is_ok() {
        return ColorMode::NoColor;
    }

    let colorterm = env::var("COLORTERM").unwrap_or_default().to_lowercase();
    if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        return ColorMode::TrueColor;
    }

    let term = env::var("TERM").unwrap_or_default().to_lowercase();
    if term.contains("256color") {
        return ColorMode::Ansi256;
    }

    ColorMode::NoColor
}
