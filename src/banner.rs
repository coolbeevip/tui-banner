use crate::color::ColorMode;
use crate::emit::emit_ansi;
use crate::effects::dither::apply_dot_dither;
use crate::effects::outline::{apply_edge_shade, EdgeShade};
use crate::effects::shadow::{apply_shadow, Shadow};
use crate::fill::{apply_fill, Dither, Fill};
use crate::font::{render_text, Font};
use crate::gradient::Gradient;
use crate::grid::{Align, Grid, Padding};
use crate::terminal::detect_color_mode;

#[derive(Clone, Debug)]
pub struct Banner {
    text: String,
    font: Font,
    gradient: Option<Gradient>,
    fill: Fill,
    shadow: Option<Shadow>,
    edge_shade: Option<EdgeShade>,
    dot_dither: Option<Dither>,
    dot_dither_targets: Option<Vec<char>>,
    align: Align,
    padding: Padding,
    width: Option<usize>,
    max_width: Option<usize>,
    kerning: usize,
    line_gap: usize,
    color_mode: ColorMode,
}

impl Banner {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font: Font::block(),
            gradient: None,
            fill: Fill::Blocks,
            shadow: None,
            edge_shade: None,
            dot_dither: None,
            dot_dither_targets: None,
            align: Align::Left,
            padding: Padding::uniform(0),
            width: None,
            max_width: None,
            kerning: 1,
            line_gap: 0,
            color_mode: ColorMode::Auto,
        }
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    pub fn gradient(mut self, gradient: Gradient) -> Self {
        self.gradient = Some(gradient);
        self
    }

    pub fn fill(mut self, fill: Fill) -> Self {
        self.fill = fill;
        self
    }

    pub fn shadow(mut self, offset: (i32, i32), alpha: f32) -> Self {
        self.shadow = Some(Shadow { offset, alpha });
        self
    }

    pub fn edge_shade(mut self, darken: f32, ch: char) -> Self {
        self.edge_shade = Some(EdgeShade { ch, darken });
        self
    }

    pub fn dot_dither(mut self, dither: Dither) -> Self {
        self.dot_dither = Some(dither);
        self
    }

    pub fn dot_dither_targets(mut self, targets: &[char]) -> Self {
        self.dot_dither_targets = Some(targets.to_vec());
        self
    }

    pub fn dot_dither_targets_str(mut self, targets: &str) -> Self {
        self.dot_dither_targets = Some(targets.chars().collect());
        self
    }

    pub fn dither(self) -> DotDitherBuilder {
        DotDitherBuilder::new(self)
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    pub fn max_width(mut self, width: usize) -> Self {
        self.max_width = Some(width);
        self
    }

    pub fn kerning(mut self, kerning: usize) -> Self {
        self.kerning = kerning;
        self
    }

    pub fn line_gap(mut self, line_gap: usize) -> Self {
        self.line_gap = line_gap;
        self
    }

    pub fn color_mode(mut self, mode: ColorMode) -> Self {
        self.color_mode = mode;
        self
    }

    pub fn render(&self) -> String {
        let mut grid = render_text(&self.text, &self.font, self.kerning, self.line_gap);
        apply_fill(&mut grid, self.fill);
        if let Some(gradient) = &self.gradient {
            gradient.apply(&mut grid);
        }
        if let Some(dither) = self.dot_dither {
            let default_targets = ['░', '▒'];
            let targets = self
                .dot_dither_targets
                .as_deref()
                .unwrap_or(&default_targets);
            grid = apply_dot_dither(&grid, dither, targets);
        }
        if let Some(shade) = self.edge_shade {
            grid = apply_edge_shade(&grid, shade);
        }
        if let Some(shadow) = self.shadow {
            grid = apply_shadow(&grid, shadow);
        }
        let grid = apply_layout(grid, self.padding, self.width, self.max_width, self.align);
        let mode = match self.color_mode {
            ColorMode::Auto => detect_color_mode(),
            other => other,
        };
        emit_ansi(&grid, mode)
    }
}

pub struct DotDitherBuilder {
    banner: Banner,
    targets: Vec<char>,
    dots: (char, char),
}

impl DotDitherBuilder {
    fn new(banner: Banner) -> Self {
        Self {
            banner,
            targets: vec!['░', '▒'],
            dots: ('·', ':'),
        }
    }

    pub fn targets(mut self, targets: &str) -> Self {
        self.targets = targets.chars().collect();
        self
    }

    pub fn targets_vec(mut self, targets: &[char]) -> Self {
        self.targets = targets.to_vec();
        self
    }

    pub fn dots(mut self, dots: &str) -> Self {
        self.dots = parse_dots(dots);
        self
    }

    pub fn checker(mut self, period: u8) -> Banner {
        let dither = Dither {
            mode: crate::fill::DitherMode::Checker { period },
            dot: self.dots.0,
            alt: self.dots.1,
        };
        self.banner = self
            .banner
            .dot_dither(dither)
            .dot_dither_targets(&self.targets);
        self.banner
    }

    pub fn noise(mut self, seed: u32, threshold: u8) -> Banner {
        let dither = Dither {
            mode: crate::fill::DitherMode::Noise { seed, threshold },
            dot: self.dots.0,
            alt: self.dots.1,
        };
        self.banner = self
            .banner
            .dot_dither(dither)
            .dot_dither_targets(&self.targets);
        self.banner
    }
}

fn parse_dots(dots: &str) -> (char, char) {
    let mut iter = dots.chars();
    let first = iter.next().unwrap_or('·');
    let second = iter.next().unwrap_or(first);
    (first, second)
}

fn apply_layout(
    mut grid: Grid,
    padding: Padding,
    width: Option<usize>,
    max_width: Option<usize>,
    align: Align,
) -> Grid {
    let height = grid.height();
    let width_now = grid.width();
    let padded_width = width_now + padding.left + padding.right;
    let padded_height = height + padding.top + padding.bottom;

    let mut padded = Grid::new(padded_height, padded_width);
    padded.blit(&grid, padding.top, padding.left);
    grid = padded;

    let mut target_width = width;
    if let Some(max_width) = max_width {
        let limit = grid.width().min(max_width);
        target_width = Some(target_width.map_or(limit, |w| w.min(max_width)));
    }

    if let Some(target) = target_width {
        if target > grid.width() {
            let extra = target - grid.width();
            let left_extra = match align {
                Align::Left => 0,
                Align::Center => extra / 2,
                Align::Right => extra,
            };
            let right_extra = extra - left_extra;
            let mut expanded = Grid::new(grid.height(), target);
            expanded.blit(&grid, 0, left_extra);
            if right_extra > 0 {
                // already blank by default
            }
            grid = expanded;
        } else if target < grid.width() {
            grid = clip_width(&grid, target, align);
        }
    }

    grid
}

fn clip_width(grid: &Grid, target: usize, align: Align) -> Grid {
    if target == 0 {
        return Grid::new(grid.height(), 0);
    }

    let start = match align {
        Align::Left => 0,
        Align::Center => (grid.width().saturating_sub(target)) / 2,
        Align::Right => grid.width().saturating_sub(target),
    };

    let mut out = Grid::new(grid.height(), target);
    for r in 0..grid.height() {
        for c in 0..target {
            if let Some(cell) = grid.cell(r, start + c) {
                if let Some(target_cell) = out.cell_mut(r, c) {
                    *target_cell = cell.clone();
                }
            }
        }
    }
    out
}
