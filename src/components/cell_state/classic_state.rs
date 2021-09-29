use crate::components::CellState;
#[cfg(feature = "auto-coloring")]
use crate::resources::materials::CellStateMaterials;
#[cfg(feature = "auto-coloring")]
use bevy::prelude::{Assets, Color, ColorMaterial};

pub type ClassicCellState = bool;

impl CellState for ClassicCellState {
    fn new_cell_state(&self, neighbor_cells: &[Self]) -> Self {
        let alive_cells_count = neighbor_cells.iter().filter(|&c| *c).count();
        if *self {
            (2..=3).contains(&alive_cells_count)
        } else {
            alive_cells_count == 3
        }
    }

    #[cfg(feature = "auto-coloring")]
    fn material_index(&self) -> usize {
        if *self {
            1
        } else {
            0
        }
    }

    #[cfg(feature = "auto-coloring")]
    fn setup_materials(materials: &mut Assets<ColorMaterial>) -> CellStateMaterials {
        CellStateMaterials {
            materials: vec![
                materials.add(Color::BLACK.into()),
                materials.add(Color::WHITE.into()),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overpopulation_rule() {
        let cell_state = true;

        // 4 alive neighbors
        let neighbors = vec![false, true, false, true, true, true, false, false];
        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(!new_state);
        // 8 alive neighbors
        let neighbors = vec![true, true, true, true, true, true, true, true];
        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(!new_state);
    }

    #[test]
    fn generation_rule() {
        let cell_state = true;

        // 3 alive neighbors
        let neighbors = vec![false, true, false, true, false, true, false, false];
        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(new_state);

        // 2 alive neighbors
        let neighbors = vec![false, true, false, false, false, true, false, false];
        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(new_state);

        // 2 alive neighbors but "off"
        let cell_state = false;
        let neighbors = vec![false, true, false, false, false, true, false, false];
        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(!new_state);
    }

    #[test]
    fn reproduction_rule() {
        let cell_state = false;

        // 3 alive neighbors
        let neighbors = vec![false, true, false, true, false, true, false, false];
        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(new_state);
    }

    #[test]
    fn underpopulation_rule() {
        let cell_state = true;

        // 1 alive neighbors
        let neighbors = vec![false, false, false, false, false, true, false, false];
        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(!new_state);

        // 0 alive neighbors
        let neighbors = vec![false, false, false, false, false, false, false, false];
        let new_state = cell_state.new_cell_state(&neighbors);
        assert!(!new_state);
    }
}
