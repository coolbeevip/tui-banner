use crate::color::{Color, ColorMode};
use crate::grid::Grid;
use crate::terminal::detect_color_mode;

/// Emit ANSI-colored output from a grid.
pub fn emit_ansi(grid: &Grid, color_mode: ColorMode) -> String {
    let mode = match color_mode {
        ColorMode::Auto => detect_color_mode(),
        other => other,
    };

    let mut out = String::new();
    let mut current_fg: Option<Color> = None;

    for (row_idx, row) in grid.rows().iter().enumerate() {
        for cell in row {
            match mode {
                ColorMode::NoColor => {
                    out.push(cell.ch);
                }
                _ => {
                    if cell.fg != current_fg {
                        if let Some(color) = cell.fg {
                            push_fg_code(&mut out, color, mode);
                        } else {
                            out.push_str("\x1b[0m");
                        }
                        current_fg = cell.fg;
                    }
                    out.push(cell.ch);
                }
            }
        }

        if mode != ColorMode::NoColor && current_fg.is_some() {
            out.push_str("\x1b[0m");
            current_fg = None;
        }

        if row_idx + 1 < grid.height() {
            out.push('\n');
        }
    }

    out
}

fn push_fg_code(out: &mut String, color: Color, mode: ColorMode) {
    match mode {
        ColorMode::TrueColor => match color {
            Color::Rgb(r, g, b) => {
                out.push_str(&format!("\x1b[38;2;{};{};{}m", r, g, b));
            }
            Color::Ansi256(code) => {
                out.push_str(&format!("\x1b[38;5;{}m", code));
            }
        },
        ColorMode::Ansi256 => {
            let code = match color {
                Color::Ansi256(v) => v,
                Color::Rgb(r, g, b) => rgb_to_ansi256(r, g, b),
            };
            out.push_str(&format!("\x1b[38;5;{}m", code));
        }
        _ => {}
    }
}

fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    if r == g && g == b {
        if r < 8 {
            return 16;
        }
        if r > 248 {
            return 231;
        }
        return 232 + ((r as u16 - 8) / 10) as u8;
    }

    let rc = (r as u16 * 5 / 255) as u8;
    let gc = (g as u16 * 5 / 255) as u8;
    let bc = (b as u16 * 5 / 255) as u8;
    16 + 36 * rc + 6 * gc + bc
}
