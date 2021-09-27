use crate::components::CellState;

pub type ClassicCellState = bool;

impl CellState for ClassicCellState {
    fn new_cell_state(&self, neighbor_cells: &Vec<Self>) -> Self {
        let alive_cells_count = neighbor_cells.iter().filter(|&c| *c).count();
        !(alive_cells_count < 2 || alive_cells_count > 3)
    }
}
