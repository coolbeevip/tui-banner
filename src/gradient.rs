use crate::color::{Color, Palette};
use crate::grid::Grid;

/// Gradient definition for coloring a grid.
#[derive(Clone, Debug)]
pub struct Gradient {
    stops: Vec<Color>,
    direction: GradientDirection,
}

/// Gradient direction.
#[derive(Clone, Copy, Debug)]
pub enum GradientDirection {
    /// Top to bottom.
    Vertical,
    /// Left to right.
    Horizontal,
    /// Top-left to bottom-right.
    Diagonal,
}

impl Gradient {
    /// Create a gradient from color stops and direction.
    pub fn new(stops: Vec<Color>, direction: GradientDirection) -> Self {
        Self { stops, direction }
    }

    /// Vertical gradient (top -> bottom).
    pub fn vertical(palette: Palette) -> Self {
        Self::new(palette.colors().to_vec(), GradientDirection::Vertical)
    }

    /// Horizontal gradient (left -> right).
    pub fn horizontal(palette: Palette) -> Self {
        Self::new(palette.colors().to_vec(), GradientDirection::Horizontal)
    }

    /// Diagonal gradient (top-left -> bottom-right).
    pub fn diagonal(palette: Palette) -> Self {
        Self::new(palette.colors().to_vec(), GradientDirection::Diagonal)
    }

    /// Apply the gradient to a grid in-place.
    pub fn apply(&self, grid: &mut Grid) {
        if self.stops.is_empty() {
            return;
        }

        let height = grid.height().max(1);
        let width = grid.width().max(1);

        for r in 0..height {
            for c in 0..width {
                let t = match self.direction {
                    GradientDirection::Vertical => {
                        if height <= 1 {
                            0.0
                        } else {
                            r as f32 / (height - 1) as f32
                        }
                    }
                    GradientDirection::Horizontal => {
                        if width <= 1 {
                            0.0
                        } else {
                            c as f32 / (width - 1) as f32
                        }
                    }
                    GradientDirection::Diagonal => {
                        if width + height <= 2 {
                            0.0
                        } else {
                            (r + c) as f32 / (width + height - 2) as f32
                        }
                    }
                };

                if let Some(cell) = grid.cell_mut(r, c)
                    && cell.visible
                {
                    cell.fg = Some(color_at(&self.stops, t));
                }
            }
        }
    }
}

fn color_at(stops: &[Color], t: f32) -> Color {
    if stops.len() == 1 {
        return stops[0];
    }

    let t = t.clamp(0.0, 1.0);
    let max_index = stops.len() - 1;
    let scaled = t * max_index as f32;
    let idx = scaled.floor() as usize;
    let next = idx.min(max_index - 1) + 1;
    let local_t = scaled - idx as f32;

    stops[idx].lerp(stops[next], local_t)
}
