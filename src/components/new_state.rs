use crate::{CellState, ClassicCellState, WorldWireCellState};

pub type NewClassicCellState = NewState<ClassicCellState>;
pub type NewWireWorldCellState = NewState<WorldWireCellState>;

#[derive(Debug)]
pub struct NewState<T: CellState>(pub T);

impl NewClassicCellState {
    pub fn new(value: bool) -> Self {
        Self(value)
    }
}

impl NewWireWorldCellState {
    pub fn new(state: WorldWireCellState) -> Self {
        Self(state)
    }
}
