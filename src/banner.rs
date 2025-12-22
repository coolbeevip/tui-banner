// Copyright (c) 2025 Lei Zhang
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crate::color::Palette;
use crate::color::{Color, ColorMode};
use crate::effects::dither::apply_dot_dither;
use crate::effects::light_sweep::{LightSweep, SweepDirection, apply_light_sweep_tint};
use crate::effects::outline::{EdgeShade, apply_edge_shade};
use crate::effects::shadow::{Shadow, apply_shadow};
use crate::emit::emit_ansi;
use crate::fill::{Dither, Fill, apply_fill};
use crate::font::{self, Font, render_text};
use crate::gradient::Gradient;
use crate::grid::{Align, Grid, Padding};
use crate::style::Style;
use crate::terminal::detect_color_mode;

/// High-level banner builder.
#[derive(Clone, Debug)]
pub struct Banner {
    text: String,
    font: Font,
    gradient: Option<Gradient>,
    fill: Fill,
    light_sweep: Option<LightSweep>,
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

/// Errors returned when building a banner.
#[derive(Debug)]
pub enum BannerError {
    /// Failed to parse the bundled Figlet font.
    Font(font::figlet::FigletError),
}

impl std::fmt::Display for BannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BannerError::Font(err) => write!(f, "font parse error: {err:?}"),
        }
    }
}

impl std::error::Error for BannerError {}

impl From<font::figlet::FigletError> for BannerError {
    fn from(err: font::figlet::FigletError) -> Self {
        BannerError::Font(err)
    }
}

impl Banner {
    /// Create a banner from text.
    ///
    /// Returns an error if the bundled font cannot be parsed.
    pub fn new(text: impl Into<String>) -> Result<Self, BannerError> {
        Ok(Self {
            text: text.into(),
            font: Font::dos_rebel()?,
            gradient: None,
            fill: Fill::Blocks,
            light_sweep: None,
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
        })
    }

    /// Set the font.
    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    /// Apply a named style preset.
    pub fn style(mut self, style: Style) -> Self {
        self.color_mode = ColorMode::TrueColor;
        self.gradient = Some(Gradient::vertical(Palette::preset(style.preset())));
        self.fill = Fill::Keep;
        self
    }

    /// Apply a gradient across the glyph grid.
    pub fn gradient(mut self, gradient: Gradient) -> Self {
        self.gradient = Some(gradient);
        self
    }

    /// Fill visible cells (or keep glyph characters).
    pub fn fill(mut self, fill: Fill) -> Self {
        self.fill = fill;
        self
    }

    /// Add a drop shadow.
    pub fn shadow(mut self, offset: (i32, i32), alpha: f32) -> Self {
        self.shadow = Some(Shadow { offset, alpha });
        self
    }

    /// Add a highlight sweep (useful for animated passes).
    pub fn light_sweep(mut self, sweep: LightSweep) -> Self {
        self.light_sweep = Some(sweep);
        self
    }

    /// Add a 1-cell edge shade using a darker color and a dedicated character.
    pub fn edge_shade(mut self, darken: f32, ch: char) -> Self {
        self.edge_shade = Some(EdgeShade { ch, darken });
        self
    }

    /// Enable dot dithering using a custom configuration.
    pub fn dot_dither(mut self, dither: Dither) -> Self {
        self.dot_dither = Some(dither);
        self
    }

    /// Set the dither targets (glyphs to be replaced by dots).
    pub fn dot_dither_targets(mut self, targets: &[char]) -> Self {
        self.dot_dither_targets = Some(targets.to_vec());
        self
    }

    /// Set the dither targets using a string (e.g. "░▒▓").
    pub fn dot_dither_targets_str(mut self, targets: &str) -> Self {
        self.dot_dither_targets = Some(targets.chars().collect());
        self
    }

    /// Builder-style dot dithering configuration.
    pub fn dither(self) -> DotDitherBuilder {
        DotDitherBuilder::new(self)
    }

    /// Align within the target width.
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Add padding around the banner.
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Force an output width (pads or clips).
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Clamp output width.
    pub fn max_width(mut self, width: usize) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Space between characters.
    pub fn kerning(mut self, kerning: usize) -> Self {
        self.kerning = kerning;
        self
    }

    /// Blank lines between text lines.
    pub fn line_gap(mut self, line_gap: usize) -> Self {
        self.line_gap = line_gap;
        self
    }

    /// Override color mode.
    pub fn color_mode(mut self, mode: ColorMode) -> Self {
        self.color_mode = mode;
        self
    }

    /// Render to a `String` (ANSI escapes included if enabled).
    pub fn render(&self) -> String {
        self.render_with_sweep(None, None)
    }

    /// Animate a light sweep over the banner.
    ///
    /// `speed_ms` controls the delay between frames in milliseconds.
    /// `highlight` overrides the sweep color (use `None` for white).
    pub fn animate_sweep(&self, speed_ms: u64, highlight: Option<Color>) -> io::Result<()> {
        let mut stdout = io::stdout();
        write!(stdout, "\x1b[2J\x1b[?25l")?;
        stdout.flush()?;

        let frames = 180;
        let frame_time = Duration::from_millis(speed_ms);
        let highlight = highlight.unwrap_or(Color::Rgb(255, 255, 255));
        let base = self.light_sweep.unwrap_or_else(|| {
            LightSweep::new(SweepDirection::DiagonalDown)
                .width(0.25)
                .intensity(0.9)
                .softness(2.5)
        });
        let start = base.center - 0.75;
        let end = base.center + 0.75;
        for frame in 0..frames {
            let t = frame as f32 / frames as f32;
            let center = start + t * (end - start);
            let sweep = base.center(center);

            let banner = self.render_with_sweep(Some(sweep), Some(highlight));
            write!(stdout, "\x1b[H{banner}")?;
            stdout.flush()?;
            thread::sleep(frame_time);
        }

        writeln!(stdout, "\x1b[?25h")?;
        Ok(())
    }

    fn render_with_sweep(
        &self,
        sweep_override: Option<LightSweep>,
        highlight: Option<Color>,
    ) -> String {
        let mut grid = render_text(&self.text, &self.font, self.kerning, self.line_gap);
        apply_fill(&mut grid, self.fill);
        if let Some(gradient) = &self.gradient {
            gradient.apply(&mut grid);
        }
        if let Some(sweep) = sweep_override.or(self.light_sweep) {
            let highlight = highlight.unwrap_or(Color::Rgb(255, 255, 255));
            apply_light_sweep_tint(&mut grid, sweep, highlight);
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

/// Builder for dot dithering over selected glyph targets.
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
            dots: ('░', '░'),
        }
    }

    /// Set glyphs to be replaced by dots.
    pub fn targets(mut self, targets: &str) -> Self {
        self.targets = targets.chars().collect();
        self
    }

    /// Set glyphs to be replaced by dots.
    pub fn targets_vec(mut self, targets: &[char]) -> Self {
        self.targets = targets.to_vec();
        self
    }

    /// Set dot characters (1 or 2 chars, e.g. "·:").
    pub fn dots(mut self, dots: &str) -> Self {
        self.dots = parse_dots(dots);
        self
    }

    /// Apply a checkerboard-style dither.
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

    /// Apply a hash-noise dither.
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
            if let (Some(cell), Some(target_cell)) = (grid.cell(r, start + c), out.cell_mut(r, c)) {
                *target_cell = cell.clone();
            }
        }
    }
    out
}
