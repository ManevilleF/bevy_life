pub use {classic_state::*, world_wire_state::*};

mod classic_state;
mod world_wire_state;

pub trait CellState: Sized {
    fn new_cell_state(&self, neighbor_cells: &Vec<Self>) -> Self;

    fn apply_new_cell_state(&mut self, neighbor_cells: &Vec<Self>) {
        *self = self.new_cell_state(neighbor_cells)
    }
}
