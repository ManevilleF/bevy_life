use crate::components::CellState;
use bevy::prelude::Component;
#[cfg(feature = "auto-coloring")]
use bevy::render::color::Color;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Component)]
/// Classic cellular automation state and rules following Conway's game of life variation: The immigration game.
///
/// - Any live cell with fewer than two live neighbours dies, as if by underpopulation.
/// - Any live cell with two or three live neighbours lives on to the next generation.
/// - Any live cell with more than three live neighbours dies, as if by overpopulation.
/// - Any dead cell with exactly three live neighbours becomes a live cell,
/// as if by reproduction and takes the arithmetic mean state of the majority of the live neighbors.
///
pub enum RainbowCellState {
    /// A dead cell
    Dead,
    /// Alive cell with a `f32` sub-state
    Alive(f32),
}

impl CellState for RainbowCellState {
    fn new_cell_state(&self, neighbor_cells: &[Self]) -> Self {
        let alive_cells: Vec<f32> = neighbor_cells
            .iter()
            .filter_map(|c| match c {
                Self::Dead => None,
                Self::Alive(s) => Some(*s),
            })
            .collect();
        let alive_cells_count = alive_cells.len();
        if self.is_alive() {
            if (2..=3).contains(&alive_cells_count) {
                self.clone()
            } else {
                Self::Dead
            }
        } else if alive_cells_count == 3 {
            let val: f32 = alive_cells.into_iter().sum::<f32>() / 3.;
            Self::Alive(val)
        } else {
            Self::Dead
        }
    }

    #[cfg(feature = "auto-coloring")]
    fn color(&self) -> Option<Color> {
        match self {
            Self::Dead => None,
            Self::Alive(v) => Some(Color::rgb(*v, *v, *v)),
        }
    }
}

impl RainbowCellState {
    /// Is the cell considered alive
    pub const fn is_alive(&self) -> bool {
        matches!(self, Self::Alive(_))
    }
}

impl Default for RainbowCellState {
    fn default() -> Self {
        Self::Dead
    }
}
