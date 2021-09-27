use crate::CellState;

#[derive(Debug)]
pub struct NewState<T: CellState>(pub T);
