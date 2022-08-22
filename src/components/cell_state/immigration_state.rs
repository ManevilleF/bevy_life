use crate::components::CellState;
use bevy::prelude::{Component, Reflect};
#[cfg(feature = "auto-coloring")]
use bevy::render::color::Color;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Component, Reflect)]
/// Classic cellular automation state and rules following Conway's game of life variation: The immigration game.
///
/// - Any live cell with fewer than two live neighbours dies, as if by underpopulation.
/// - Any live cell with two or three live neighbours lives on to the next generation.
/// - Any live cell with more than three live neighbours dies, as if by overpopulation.
/// - Any dead cell with exactly three live neighbours becomes a live cell,
/// as if by reproduction and takes the state of the majority of the live neighbors.
///
pub enum ImmigrationCellState {
    /// A dead cell
    Dead,
    /// Alive cell with a boolean sub-state
    Alive(bool),
}

impl CellState for ImmigrationCellState {
    fn new_cell_state(&self, neighbor_cells: &[Self]) -> Self {
        let alive_cells: Vec<bool> = neighbor_cells
            .iter()
            .filter_map(|c| match c {
                Self::Dead => None,
                Self::Alive(s) => Some(*s),
            })
            .collect();
        let alive_cells_count = alive_cells.len();
        if self.is_alive() {
            if (2..=3).contains(&alive_cells_count) {
                *self
            } else {
                Self::Dead
            }
        } else if alive_cells_count == 3 {
            let mut map = HashMap::new();
            for alive_cell in alive_cells {
                *map.entry(alive_cell).or_insert(0) += 1;
            }
            Self::Alive(
                map.into_iter()
                    .max_by_key(|(_k, v)| *v)
                    .map(|(k, _v)| k)
                    .unwrap(),
            )
        } else {
            Self::Dead
        }
    }

    #[cfg(feature = "auto-coloring")]
    fn color(&self) -> Option<Color> {
        match self {
            Self::Dead => None,
            Self::Alive(b) => Some(if *b { Color::CYAN } else { Color::ORANGE }),
        }
    }
}

impl ImmigrationCellState {
    /// Is the cell considered alive
    #[must_use]
    #[inline]
    pub const fn is_alive(&self) -> bool {
        matches!(self, Self::Alive(_))
    }
}

impl Default for ImmigrationCellState {
    fn default() -> Self {
        Self::Dead
    }
}
