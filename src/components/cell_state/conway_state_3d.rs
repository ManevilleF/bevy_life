use crate::components::CellState;
#[cfg(feature = "auto-coloring")]
use crate::ColorResponse;
#[cfg(feature = "auto-coloring")]
use bevy::prelude::Color;
use std::ops::{Deref, DerefMut};

/// Classic cellular automation state and rules following Conway's game of life **4555** rules:
///
/// - Any live cell with fewer than four live neighbours dies, as if by underpopulation.
/// - Any live cell with two or five live neighbours lives on to the next generation.
/// - Any live cell with more than five live neighbours dies, as if by overpopulation.
/// - Any dead cell with exactly five live neighbours becomes a live cell, as if by reproduction.
///
/// A dead cell is `false`, a live cell is `true`
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ConwayCell4555State(pub bool);

impl CellState for ConwayCell4555State {
    fn new_cell_state(&self, neighbor_cells: &[Self]) -> Self {
        let alive_cells_count = neighbor_cells.iter().filter(|&c| c.0).count();
        if self.0 {
            Self((4..=5).contains(&alive_cells_count))
        } else {
            Self(alive_cells_count == 5)
        }
    }

    #[cfg(feature = "auto-coloring")]
    fn color_or_material_index(&self) -> ColorResponse {
        if self.0 {
            ColorResponse::MaterialIndex(1)
        } else {
            ColorResponse::None
        }
    }

    #[cfg(feature = "auto-coloring")]
    fn colors() -> &'static [Color] {
        &[Color::BLACK, Color::WHITE]
    }
}

impl Deref for ConwayCell4555State {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConwayCell4555State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<bool> for ConwayCell4555State {
    fn from(val: bool) -> Self {
        Self(val)
    }
}
