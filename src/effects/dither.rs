use crate::fill::{Dither, DitherMode};
use crate::grid::Grid;

/// Apply dot dithering over selected glyph targets.
pub fn apply_dot_dither(grid: &Grid, dither: Dither, targets: &[char]) -> Grid {
    let mut out = grid.clone();
    let height = out.height();
    let width = out.width();

    for r in 0..height {
        for c in 0..width {
            let Some(cell) = out.cell_mut(r, c) else {
                continue;
            };
            if !cell.visible {
                continue;
            }
            if !targets.contains(&cell.ch) {
                continue;
            }
            if should_dither(r, c, dither.mode) {
                cell.ch = if (r + c) % 2 == 0 {
                    dither.dot
                } else {
                    dither.alt
                };
            }
        }
    }

    out
}

fn should_dither(row: usize, col: usize, mode: DitherMode) -> bool {
    match mode {
        DitherMode::Checker { period } => {
            if period == 0 {
                false
            } else {
                (row + col).is_multiple_of(period as usize)
            }
        }
        DitherMode::Noise { seed, threshold } => {
            let hash = mix(seed, row as u32, col as u32);
            (hash & 0xFF) < threshold as u32
        }
    }
}

fn mix(seed: u32, x: u32, y: u32) -> u32 {
    let mut v = seed ^ x.wrapping_mul(0x9E3779B1) ^ y.wrapping_mul(0x85EBCA77);
    v ^= v >> 16;
    v = v.wrapping_mul(0x7FEB352D);
    v ^= v >> 15;
    v = v.wrapping_mul(0x846CA68B);
    v ^= v >> 16;
    v
}
