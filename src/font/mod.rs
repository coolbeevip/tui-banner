use std::collections::HashMap;

use crate::grid::Grid;

mod builtin;
/// Figlet font parser.
pub mod figlet;

/// A single glyph as character rows.
#[derive(Clone, Debug)]
pub struct Glyph {
    rows: Vec<Vec<char>>,
}

/// Font containing glyphs and height.
#[derive(Clone, Debug)]
pub struct Font {
    height: usize,
    glyphs: HashMap<char, Glyph>,
    fallback: Glyph,
}

impl Font {
    /// Built-in 5x5 block font.
    pub fn block() -> Self {
        builtin::block_font()
    }

    /// Built-in DOS Rebel (Figlet) font.
    pub fn dos_rebel() -> Self {
        figlet::parse(include_str!("../../assets/fonts/dosrebel.flf"))
            .unwrap_or_else(|_| builtin::block_font())
    }

    /// Parse a Figlet `.flf` string into a font.
    pub fn from_figlet_str(data: &str) -> Result<Self, figlet::FigletError> {
        figlet::parse(data)
    }

    /// Font height in rows.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get glyph by character (falls back if missing).
    pub fn glyph(&self, ch: char) -> &Glyph {
        self.glyphs.get(&ch).unwrap_or(&self.fallback)
    }
}

impl Glyph {
    /// Width of the glyph.
    pub fn width(&self) -> usize {
        self.rows.first().map(|r| r.len()).unwrap_or(0)
    }
}

/// Render text into a grid using a font.
pub fn render_text(text: &str, font: &Font, kerning: usize, line_gap: usize) -> Grid {
    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return Grid::new(0, 0);
    }

    let mut line_grids = Vec::with_capacity(lines.len());
    let mut max_width = 0;

    for line in &lines {
        let grid = render_line(line, font, kerning);
        max_width = max_width.max(grid.width());
        line_grids.push(grid);
    }

    let mut rows: Vec<Vec<char>> = Vec::new();
    for (idx, grid) in line_grids.into_iter().enumerate() {
        for row in grid.rows() {
            let mut chars = row.iter().map(|cell| cell.ch).collect::<Vec<_>>();
            if chars.len() < max_width {
                chars.extend(std::iter::repeat(' ').take(max_width - chars.len()));
            }
            rows.push(chars);
        }
        if idx + 1 < lines.len() {
            for _ in 0..line_gap {
                rows.push(vec![' '; max_width]);
            }
        }
    }

    Grid::from_char_rows(rows)
}

fn render_line(text: &str, font: &Font, kerning: usize) -> Grid {
    let mut rows: Vec<Vec<char>> = vec![Vec::new(); font.height()];
    let chars: Vec<char> = text.chars().collect();

    for (idx, ch) in chars.iter().enumerate() {
        let glyph = font.glyph(ch.to_ascii_uppercase());
        for (row_idx, row) in glyph.rows.iter().enumerate() {
            rows[row_idx].extend(row.iter().copied());
            if idx + 1 < chars.len() && kerning > 0 {
                rows[row_idx].extend(std::iter::repeat(' ').take(kerning));
            }
        }
    }

    Grid::from_char_rows(rows)
}
