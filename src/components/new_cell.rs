use crate::cell::Cell;
use std::fmt::Debug;

#[derive(Debug)]
pub struct NewCell<T: Cell + Debug>(pub T);
