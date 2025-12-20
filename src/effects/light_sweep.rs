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

use crate::color::Color;
use crate::grid::Grid;

/// Direction of the light sweep.
#[derive(Clone, Copy, Debug)]
pub enum SweepDirection {
    /// Sweep left to right.
    Horizontal,
    /// Sweep top to bottom.
    Vertical,
    /// Sweep from top-left to bottom-right.
    DiagonalDown,
    /// Sweep from bottom-left to top-right.
    DiagonalUp,
}

/// Highlight sweep configuration.
#[derive(Clone, Copy, Debug)]
pub struct LightSweep {
    /// Center position along the sweep axis (0.0..1.0, values outside are allowed for off-screen).
    pub center: f32,
    /// Width of the sweep band along the axis (0.0..1.0).
    pub width: f32,
    /// Peak brightening amount (0.0..1.0).
    pub intensity: f32,
    /// Falloff curve exponent (>= 1.0).
    pub softness: f32,
    /// Sweep direction.
    pub direction: SweepDirection,
}

impl LightSweep {
    /// Create a sweep with sensible defaults.
    pub fn new(direction: SweepDirection) -> Self {
        Self {
            center: 0.5,
            width: 0.25,
            intensity: 0.8,
            softness: 2.0,
            direction,
        }
    }

    /// Set the sweep center (0.0..1.0).
    pub fn center(mut self, center: f32) -> Self {
        self.center = center;
        self
    }

    /// Set the sweep width (0.0..1.0).
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the sweep intensity (0.0..1.0).
    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity;
        self
    }

    /// Set the sweep falloff exponent (>= 1.0).
    pub fn softness(mut self, softness: f32) -> Self {
        self.softness = softness;
        self
    }
}

/// Apply a highlight sweep in-place.
pub fn apply_light_sweep(grid: &mut Grid, sweep: LightSweep) {
    apply_light_sweep_tint(grid, sweep, Color::Rgb(255, 255, 255));
}

/// Apply a highlight sweep in-place with a custom highlight color.
pub fn apply_light_sweep_tint(grid: &mut Grid, sweep: LightSweep, highlight: Color) {
    let height = grid.height().max(1);
    let width = grid.width().max(1);

    let intensity = sweep.intensity.clamp(0.0, 1.0);
    let band = sweep.width.max(0.0);
    if intensity <= 0.0 || band <= 0.0 {
        return;
    }

    let half = band / 2.0;
    let softness = sweep.softness.max(1.0);

    for r in 0..height {
        for c in 0..width {
            let Some(cell) = grid.cell_mut(r, c) else {
                continue;
            };
            if !cell.visible {
                continue;
            }

            let t = axis_t(sweep.direction, r, c, width, height);
            let dist = (t - sweep.center).abs();
            if dist > half {
                continue;
            }

            let falloff = 1.0 - (dist / half);
            let strength = falloff.powf(softness);
            let amount = (intensity * strength).clamp(0.0, 1.0);
            if amount <= 0.0 {
                continue;
            }

            if let Some(color) = cell.fg {
                cell.fg = Some(blend_to(color, highlight, amount));
            }
        }
    }
}

fn axis_t(direction: SweepDirection, row: usize, col: usize, width: usize, height: usize) -> f32 {
    match direction {
        SweepDirection::Horizontal => {
            if width <= 1 {
                0.0
            } else {
                col as f32 / (width - 1) as f32
            }
        }
        SweepDirection::Vertical => {
            if height <= 1 {
                0.0
            } else {
                row as f32 / (height - 1) as f32
            }
        }
        SweepDirection::DiagonalDown => {
            if width + height <= 2 {
                0.0
            } else {
                (row + col) as f32 / (width + height - 2) as f32
            }
        }
        SweepDirection::DiagonalUp => {
            if width + height <= 2 {
                0.0
            } else {
                (row + (width - 1 - col)) as f32 / (width + height - 2) as f32
            }
        }
    }
}

fn blend_to(color: Color, target: Color, amount: f32) -> Color {
    color.lerp(target, amount)
}
