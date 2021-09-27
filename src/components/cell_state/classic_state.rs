use crate::components::CellState;

pub type ClassicCellState = bool;

impl CellState for ClassicCellState {
    fn new_cell_state(&self, neighbor_cells: &[&Self]) -> Self {
        let alive_cells_count = neighbor_cells.iter().filter(|&&c| *c).count();
        !(2..=3).contains(&alive_cells_count)
    }
}
