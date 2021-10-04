use crate::components::CellState;
#[cfg(feature = "auto-coloring")]
use crate::ColorResponse;
#[cfg(feature = "auto-coloring")]
use bevy::prelude::Color;
use std::ops::{Deref, DerefMut};

/// Classic cellular automation state and rules following Conway's game of life classic **2333** rules:
///
/// - Any live cell with fewer than two live neighbours dies, as if by underpopulation.
/// - Any live cell with two or three live neighbours lives on to the next generation.
/// - Any live cell with more than three live neighbours dies, as if by overpopulation.
/// - Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
///
/// A dead cell is `false`, a live cell is `true`
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ConwayCellState(pub bool);

impl CellState for ConwayCellState {
    fn new_cell_state(&self, neighbor_cells: &[Self]) -> Self {
        let alive_cells_count = neighbor_cells.iter().filter(|&c| c.0).count();
        if self.0 {
            Self((2..=3).contains(&alive_cells_count))
        } else {
            Self(alive_cells_count == 3)
        }
    }

    #[cfg(feature = "auto-coloring")]
    fn color_or_material_index(&self) -> ColorResponse {
        if self.0 {
            ColorResponse::MaterialIndex(0)
        } else {
            ColorResponse::None
        }
    }

    #[cfg(feature = "auto-coloring")]
    fn colors() -> &'static [Color] {
        &[Color::WHITE]
    }
}

impl Deref for ConwayCellState {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConwayCellState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<bool> for ConwayCellState {
    fn from(val: bool) -> Self {
        Self(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overpopulation_rule() {
        let cell_state = ConwayCellState(true);

        // 4 alive neighbors
        let neighbors = vec![
            false.into(),
            true.into(),
            false.into(),
            true.into(),
            true.into(),
            true.into(),
            false.into(),
            false.into(),
        ];

        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(!new_state.0);
        // 8 alive neighbors
        let neighbors = vec![
            true.into(),
            true.into(),
            true.into(),
            true.into(),
            true.into(),
            true.into(),
            true.into(),
            true.into(),
        ];

        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(!new_state.0);
    }

    #[test]
    fn generation_rule() {
        let cell_state = ConwayCellState(true);

        // 3 alive neighbors
        let neighbors = vec![
            false.into(),
            true.into(),
            false.into(),
            true.into(),
            false.into(),
            true.into(),
            false.into(),
            false.into(),
        ];

        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(new_state.0);

        // 2 alive neighbors
        let neighbors = vec![
            false.into(),
            true.into(),
            false.into(),
            false.into(),
            false.into(),
            true.into(),
            false.into(),
            false.into(),
        ];

        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(new_state.0);

        // 2 alive neighbors but "off"
        let cell_state = ConwayCellState(false);
        let neighbors = vec![
            false.into(),
            true.into(),
            false.into(),
            false.into(),
            false.into(),
            true.into(),
            false.into(),
            false.into(),
        ];

        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(!new_state.0);
    }

    #[test]
    fn reproduction_rule() {
        let cell_state = ConwayCellState(false);

        // 3 alive neighbors
        let neighbors = vec![
            false.into(),
            true.into(),
            false.into(),
            true.into(),
            false.into(),
            true.into(),
            false.into(),
            false.into(),
        ];

        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(new_state.0);
    }

    #[test]
    fn underpopulation_rule() {
        let cell_state = ConwayCellState(true);

        // 1 alive neighbors
        let neighbors = vec![
            false.into(),
            false.into(),
            false.into(),
            false.into(),
            false.into(),
            true.into(),
            false.into(),
            false.into(),
        ];

        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(!new_state.0);

        // 0 alive neighbors
        let neighbors = vec![
            false.into(),
            false.into(),
            false.into(),
            false.into(),
            false.into(),
            false.into(),
            false.into(),
            false.into(),
        ];

        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(!new_state.0);
    }
}
