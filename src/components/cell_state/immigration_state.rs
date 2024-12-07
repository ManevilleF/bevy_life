use crate::components::CellState;
#[cfg(feature = "auto-coloring")]
use bevy::color::{
    palettes::css::{AQUA, ORANGE},
    Color,
};
use bevy::prelude::Component;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
/// Classic cellular automation state and rules following Conway's game of life
/// variation: The immigration game.
///
/// - Any live cell with fewer than two live neighbours dies, as if by
///   underpopulation.
/// - Any live cell with two or three live neighbours lives on to the next
///   generation.
/// - Any live cell with more than three live neighbours dies, as if by
///   overpopulation.
/// - Any dead cell with exactly three live neighbours becomes a live cell, as
///   if by reproduction and takes the state of the majority of the live
///   neighbors.
pub enum ImmigrationCellState {
    /// A dead cell
    Dead,
    /// Alive cell with a boolean sub-state
    Alive(bool),
}

impl CellState for ImmigrationCellState {
    fn new_cell_state<'a>(&self, neighbor_cells: impl Iterator<Item = &'a Self>) -> Self {
        let alive_cells: Vec<bool> = neighbor_cells
            .filter_map(|c| match c {
                Self::Dead => None,
                Self::Alive(s) => Some(*s),
            })
            .collect();
        let alive_cells_count = alive_cells.len();
        match (self, alive_cells_count) {
            (Self::Alive(_), 2 | 3) => *self,
            (Self::Dead, 3) => {
                let [mut a, mut b]: [u16; 2] = [0, 0];
                for alive_cell in alive_cells {
                    if alive_cell {
                        a += 1;
                    } else {
                        b += 1;
                    }
                }
                Self::Alive(a > b)
            }
            _ => Self::Dead,
        }
    }

    #[cfg(feature = "auto-coloring")]
    fn color(&self) -> Option<Color> {
        match self {
            Self::Dead => None,
            Self::Alive(b) => Some(if *b {
                Color::Srgba(AQUA)
            } else {
                Color::Srgba(ORANGE)
            }),
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
