use crate::color::Color;

/// Single cell in the grid.
#[derive(Clone, Debug)]
pub struct Cell {
    /// Character rendered at this cell.
    pub ch: char,
    /// Foreground color.
    pub fg: Option<Color>,
    /// Background color.
    pub bg: Option<Color>,
    /// Visibility flag (used for effects).
    pub visible: bool,
}

/// 2D grid of cells.
#[derive(Clone, Debug)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
}

/// Horizontal alignment.
#[derive(Clone, Copy, Debug)]
pub enum Align {
    /// Align to the left.
    Left,
    /// Center align.
    Center,
    /// Align to the right.
    Right,
}

/// Padding around a grid.
#[derive(Clone, Copy, Debug)]
pub struct Padding {
    /// Top padding.
    pub top: usize,
    /// Bottom padding.
    pub bottom: usize,
    /// Left padding.
    pub left: usize,
    /// Right padding.
    pub right: usize,
}

impl Grid {
    /// Create an empty grid with given dimensions.
    pub fn new(height: usize, width: usize) -> Self {
        let row = vec![
            Cell {
                ch: ' ',
                fg: None,
                bg: None,
                visible: false,
            };
            width
        ];
        let cells = vec![row; height];
        Self { cells }
    }

    /// Build a grid from raw character rows.
    pub fn from_char_rows(rows: Vec<Vec<char>>) -> Self {
        let cells = rows
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|ch| Cell {
                        ch,
                        fg: None,
                        bg: None,
                        visible: ch != ' ',
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { cells }
    }

    /// Height of the grid.
    pub fn height(&self) -> usize {
        self.cells.len()
    }

    /// Width of the grid.
    pub fn width(&self) -> usize {
        self.cells.first().map(|row| row.len()).unwrap_or(0)
    }

    /// Mutable cell access.
    pub fn cell_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.cells.get_mut(row).and_then(|r| r.get_mut(col))
    }

    /// Immutable cell access.
    pub fn cell(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(row).and_then(|r| r.get(col))
    }

    /// Borrow rows.
    pub fn rows(&self) -> &[Vec<Cell>] {
        &self.cells
    }

    /// Borrow rows mutably.
    pub fn rows_mut(&mut self) -> &mut [Vec<Cell>] {
        &mut self.cells
    }

    /// Blit another grid onto this grid at the given offset.
    pub fn blit(&mut self, other: &Grid, top: usize, left: usize) {
        for (r, row) in other.cells.iter().enumerate() {
            let target_r = top + r;
            if target_r >= self.height() {
                continue;
            }
            for (c, cell) in row.iter().enumerate() {
                let target_c = left + c;
                if target_c >= self.width() {
                    continue;
                }
                if cell.visible {
                    self.cells[target_r][target_c] = cell.clone();
                }
            }
        }
    }
}

impl Padding {
    /// Uniform padding on all sides.
    pub fn uniform(value: usize) -> Self {
        Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
        }
    }
}

impl From<usize> for Padding {
    fn from(value: usize) -> Self {
        Padding::uniform(value)
    }
}

impl From<(usize, usize, usize, usize)> for Padding {
    fn from(values: (usize, usize, usize, usize)) -> Self {
        Self {
            top: values.0,
            right: values.1,
            bottom: values.2,
            left: values.3,
        }
    }
}
