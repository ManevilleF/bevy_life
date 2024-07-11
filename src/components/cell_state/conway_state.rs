use crate::components::CellState;
#[cfg(feature = "auto-coloring")]
use bevy::prelude::Color;
use bevy::prelude::Component;
use std::ops::{Deref, DerefMut};

/// Classic cellular automation state and rules following Conway's game of life
/// classic **2333** rules:
///
/// - Any live cell with fewer than two live neighbours dies, as if by
///   underpopulation.
/// - Any live cell with two or three live neighbours lives on to the next
///   generation.
/// - Any live cell with more than three live neighbours dies, as if by
///   overpopulation.
/// - Any dead cell with exactly three live neighbours becomes a live cell, as
///   if by reproduction.
///
/// A dead cell is `false`, a live cell is `true`
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Component)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct ConwayCellState(pub bool);

impl CellState for ConwayCellState {
    fn new_cell_state<'a>(&self, neighbor_cells: impl Iterator<Item = &'a Self>) -> Self {
        let alive_cells_count = neighbor_cells.filter(|&c| c.0).count();
        let alive = matches!((self.0, alive_cells_count), (true, 2 | 3) | (false, 3));
        Self(alive)
    }

    #[cfg(feature = "auto-coloring")]
    fn color(&self) -> Option<Color> {
        if self.0 {
            Some(Color::WHITE)
        } else {
            None
        }
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
        let neighbors = [
            false.into(),
            true.into(),
            false.into(),
            true.into(),
            true.into(),
            true.into(),
            false.into(),
            false.into(),
        ];

        let new_state = cell_state.new_cell_state(neighbors.iter());
        assert!(!new_state.0);
        // 8 alive neighbors
        let neighbors = [
            true.into(),
            true.into(),
            true.into(),
            true.into(),
            true.into(),
            true.into(),
            true.into(),
            true.into(),
        ];

        let new_state = cell_state.new_cell_state(neighbors.iter());
        assert!(!new_state.0);
    }

    #[test]
    fn generation_rule() {
        let cell_state = ConwayCellState(true);

        // 3 alive neighbors
        let neighbors = [
            false.into(),
            true.into(),
            false.into(),
            true.into(),
            false.into(),
            true.into(),
            false.into(),
            false.into(),
        ];

        let new_state = cell_state.new_cell_state(neighbors.iter());
        assert!(new_state.0);

        // 2 alive neighbors
        let neighbors = [
            false.into(),
            true.into(),
            false.into(),
            false.into(),
            false.into(),
            true.into(),
            false.into(),
            false.into(),
        ];

        let new_state = cell_state.new_cell_state(neighbors.iter());
        assert!(new_state.0);

        // 2 alive neighbors but "off"
        let cell_state = ConwayCellState(false);
        let neighbors = [
            false.into(),
            true.into(),
            false.into(),
            false.into(),
            false.into(),
            true.into(),
            false.into(),
            false.into(),
        ];

        let new_state = cell_state.new_cell_state(neighbors.iter());
        assert!(!new_state.0);
    }

    #[test]
    fn reproduction_rule() {
        let cell_state = ConwayCellState(false);

        // 3 alive neighbors
        let neighbors = [
            false.into(),
            true.into(),
            false.into(),
            true.into(),
            false.into(),
            true.into(),
            false.into(),
            false.into(),
        ];

        let new_state = cell_state.new_cell_state(neighbors.iter());
        assert!(new_state.0);
    }

    #[test]
    fn underpopulation_rule() {
        let cell_state = ConwayCellState(true);

        // 1 alive neighbors
        let neighbors = [
            false.into(),
            false.into(),
            false.into(),
            false.into(),
            false.into(),
            true.into(),
            false.into(),
            false.into(),
        ];

        let new_state = cell_state.new_cell_state(neighbors.iter());
        assert!(!new_state.0);

        // 0 alive neighbors
        let neighbors = [
            false.into(),
            false.into(),
            false.into(),
            false.into(),
            false.into(),
            false.into(),
            false.into(),
            false.into(),
        ];

        let new_state = cell_state.new_cell_state(neighbors.iter());
        assert!(!new_state.0);
    }
}
