use crate::cell::Cell;

#[derive(Debug)]
pub struct NewCell<T: Cell>(pub T);
