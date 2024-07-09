use crate::CellState;
#[cfg(feature = "auto-coloring")]
use bevy::color::Color;
use bevy::prelude::Component;

/// Basic cyclic cellular automaton state and rules. The rules are the
/// following:
///
/// > As with any cellular automaton, the cyclic cellular automaton consists of
/// > a regular grid of cells in one or more dimensions.
/// > The cells can take on any of `N` states, ranging from `0`to `N − 1`. The
/// > first generation starts out with random states in each of the cells.
/// > In each subsequent generation, if a cell has a neighboring cell whose
/// > value is the successor of the cell's value, the cell is *consumed* and
/// > takes on the succeeding value.
/// > (Note that `0` is the successor of `n − 1`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Component, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct CyclicColorCellState<const N: usize>(pub usize);

impl<const N: usize> CellState for CyclicColorCellState<N> {
    fn new_cell_state<'a>(&self, neighbor_cells: impl Iterator<Item = &'a Self>) -> Self {
        let new_index = (self.0 + 1) % N;
        for neighbor_cell in neighbor_cells {
            if neighbor_cell.0 == new_index {
                return *neighbor_cell;
            }
        }
        *self
    }

    #[cfg(feature = "auto-coloring")]
    #[allow(clippy::cast_precision_loss)]
    fn color(&self) -> Option<Color> {
        let r = N as f32 / self.0 as f32;
        Some(Color::srgb(r, r, r))
    }
}

impl<const N: usize> CyclicColorCellState<N> {
    /// Return the available colors count
    #[must_use]
    #[inline]
    pub const fn max_index() -> usize {
        N
    }
}
