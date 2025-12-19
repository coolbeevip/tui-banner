use crate::color::Color;
use crate::grid::Grid;

/// Shadow configuration.
#[derive(Clone, Copy, Debug)]
pub struct Shadow {
    /// Shadow offset (dx, dy).
    pub offset: (i32, i32),
    /// Darken factor (0.0..1.0).
    pub alpha: f32,
}

/// Apply a drop shadow (darkened copy at offset).
pub fn apply_shadow(grid: &Grid, shadow: Shadow) -> Grid {
    let (dx, dy) = shadow.offset;
    if dx == 0 && dy == 0 {
        return grid.clone();
    }

    let extra_x = dx.max(0) as usize;
    let extra_y = dy.max(0) as usize;
    let mut out = Grid::new(grid.height() + extra_y, grid.width() + extra_x);
    out.blit(grid, 0, 0);

    for r in 0..grid.height() {
        for c in 0..grid.width() {
            let Some(cell) = grid.cell(r, c) else {
                continue;
            };
            if !cell.visible {
                continue;
            }
            let target_r = r as i32 + dy;
            let target_c = c as i32 + dx;
            if target_r < 0 || target_c < 0 {
                continue;
            }
            let target_r = target_r as usize;
            let target_c = target_c as usize;
            let Some(target) = out.cell_mut(target_r, target_c) else {
                continue;
            };
            if target.visible {
                continue;
            }

            target.visible = true;
            target.ch = cell.ch;
            target.fg = cell.fg.map(|color| darken(color, shadow.alpha));
        }
    }

    out
}

fn darken(color: Color, alpha: f32) -> Color {
    let factor = (1.0 - alpha.clamp(0.0, 1.0)).clamp(0.0, 1.0);
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            (r as f32 * factor).round() as u8,
            (g as f32 * factor).round() as u8,
            (b as f32 * factor).round() as u8,
        ),
        other => other,
    }
}
