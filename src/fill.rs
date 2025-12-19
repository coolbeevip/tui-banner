use crate::grid::Grid;

#[derive(Clone, Copy, Debug)]
pub enum Fill {
    Solid(char),
    Blocks,
    Keep,
    Pixel {
        block: char,
        dither: Option<Dither>,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct Dither {
    pub mode: DitherMode,
    pub dot: char,
    pub alt: char,
}

#[derive(Clone, Copy, Debug)]
pub enum DitherMode {
    Checker { period: u8 },
    Noise { seed: u32, threshold: u8 },
}

impl Dither {
    pub fn checker(period: u8, dots: &str) -> Self {
        let (dot, alt) = parse_dots(dots);
        Self {
            mode: DitherMode::Checker { period },
            dot,
            alt,
        }
    }

    pub fn noise(seed: u32, threshold: u8, dots: &str) -> Self {
        let (dot, alt) = parse_dots(dots);
        Self {
            mode: DitherMode::Noise { seed, threshold },
            dot,
            alt,
        }
    }
}

impl Fill {
    pub fn default_blocks() -> Self {
        Fill::Blocks
    }

    pub fn pixel(block: char) -> Self {
        Fill::Pixel { block, dither: None }
    }

    pub fn pixel_with_dither(block: char, dither: Dither) -> Self {
        Fill::Pixel {
            block,
            dither: Some(dither),
        }
    }
}

pub fn apply_fill(grid: &mut Grid, fill: Fill) {
    let height = grid.height();
    let width = grid.width();
    for r in 0..height {
        for c in 0..width {
            if let Some(cell) = grid.cell_mut(r, c) {
                if !cell.visible {
                    continue;
                }
                match fill {
                    Fill::Solid(ch) => {
                        cell.ch = ch;
                    }
                    Fill::Blocks => {
                        cell.ch = '#';
                    }
                    Fill::Keep => {}
                    Fill::Pixel { block, dither } => {
                        cell.ch = block;
                        if let Some(dither) = dither {
                            if should_dither(r, c, dither.mode) {
                                cell.ch = if (r + c) % 2 == 0 {
                                    dither.dot
                                } else {
                                    dither.alt
                                };
                            }
                        }
                    }
                }
            }
        }
    }
}

fn should_dither(row: usize, col: usize, mode: DitherMode) -> bool {
    match mode {
        DitherMode::Checker { period } => {
            if period == 0 {
                false
            } else {
                (row + col) % period as usize == 0
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

fn parse_dots(dots: &str) -> (char, char) {
    let mut iter = dots.chars();
    let first = iter.next().unwrap_or('·');
    let second = iter.next().unwrap_or(first);
    (first, second)
}
