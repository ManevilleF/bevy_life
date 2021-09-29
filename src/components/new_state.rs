use crate::{CellState, ClassicCellState, WireWorldCellState};

pub type NewClassicCellState = NewState<ClassicCellState>;
pub type NewWireWorldCellState = NewState<WireWorldCellState>;

#[derive(Debug)]
pub struct NewState<T: CellState>(pub T);

impl<T: CellState> NewState<T> {
    pub fn new(val: T) -> Self {
        Self(val)
    }
}
