use crate::components::CellState;
#[cfg(feature = "auto-coloring")]
use bevy::prelude::{Assets, Color};
use std::ops::{Deref, DerefMut};

/// Classic cellular automation state and rules following Conway's game of life rules:
///
/// - Any live cell with fewer than two live neighbours dies, as if by underpopulation.
/// - Any live cell with two or three live neighbours lives on to the next generation.
/// - Any live cell with more than three live neighbours dies, as if by overpopulation.
/// - Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
///
/// A dead cell is `false`, a live cell is `true`
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ClassicCellState(pub bool);

impl CellState for ClassicCellState {
    fn new_cell_state(&self, neighbor_cells: &[Self]) -> Self {
        let alive_cells_count = neighbor_cells.iter().filter(|&c| c.0).count();
        if self.0 {
            Self((2..=3).contains(&alive_cells_count))
        } else {
            Self(alive_cells_count == 3)
        }
    }

    #[cfg(feature = "auto-coloring")]
    fn material_index(&self) -> usize {
        if self.0 {
            1
        } else {
            0
        }
    }

    #[cfg(all(feature = "auto-coloring", feature = "2D"))]
    fn setup_materials_2d(
        materials: &mut Assets<bevy::prelude::ColorMaterial>,
    ) -> crate::resources::materials::CellStateMaterials2d {
        crate::resources::materials::CellStateMaterials2d {
            materials: vec![
                materials.add(Color::BLACK.into()),
                materials.add(Color::WHITE.into()),
            ],
        }
    }

    #[cfg(all(feature = "auto-coloring", feature = "3D"))]
    fn setup_materials_3d(
        materials: &mut Assets<bevy::prelude::StandardMaterial>,
    ) -> crate::resources::materials::CellStateMaterials3d {
        crate::resources::materials::CellStateMaterials3d {
            materials: vec![
                materials.add(Color::BLACK.into()),
                materials.add(Color::WHITE.into()),
            ],
        }
    }
}

impl Deref for ClassicCellState {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ClassicCellState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<bool> for ClassicCellState {
    fn from(val: bool) -> Self {
        Self(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overpopulation_rule() {
        let cell_state = ClassicCellState(true);

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
        let cell_state = ClassicCellState(true);

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
        let cell_state = ClassicCellState(false);
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
        let cell_state = ClassicCellState(false);

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
        let cell_state = ClassicCellState(true);

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
