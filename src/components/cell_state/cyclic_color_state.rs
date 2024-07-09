use crate::CellState;
#[cfg(feature = "auto-coloring")]
use bevy::color::{
    palettes::css::{AQUA, BLUE, GREEN, LIMEGREEN, ORANGE, ORANGE_RED, PURPLE, RED, YELLOW},
    Color,
};
use bevy::prelude::Component;

#[cfg(feature = "auto-coloring")]
const CYCLIC_COLORS: [Color; 9] = [
    Color::Srgba(BLUE),
    Color::Srgba(AQUA),
    Color::Srgba(GREEN),
    Color::Srgba(LIMEGREEN),
    Color::Srgba(YELLOW),
    Color::Srgba(ORANGE),
    Color::Srgba(ORANGE_RED),
    Color::Srgba(RED),
    Color::Srgba(PURPLE),
];

/// Basic cyclic cellular automaton state and rules. The rules are the
/// following:
///
/// > As with any cellular automaton, the cyclic cellular automaton consists of
/// > a regular grid of cells in one or more dimensions.
/// > The cells can take on any of `n` states, ranging from `0`to `n − 1`. The
/// > first generation starts out with random states in each of the cells.
/// > In each subsequent generation, if a cell has a neighboring cell whose
/// > value is the successor of the cell's value, the cell is *consumed* and
/// > takes on the succeeding value.
/// > (Note that `0` is the successor of `n − 1`.
///
/// For this type we use `9` for `n` and arbitrary colors.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Component, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct CyclicColorCellState(pub usize);

impl CellState for CyclicColorCellState {
    fn new_cell_state<'a>(&self, neighbor_cells: impl Iterator<Item = &'a Self>) -> Self {
        let new_index = (self.0 + 1) % CYCLIC_COLORS.len();
        for neighbor_cell in neighbor_cells {
            if neighbor_cell.0 == new_index {
                return *neighbor_cell;
            }
        }
        *self
    }

    #[cfg(feature = "auto-coloring")]
    fn color(&self) -> Option<Color> {
        Some(CYCLIC_COLORS[self.0])
    }
}

impl CyclicColorCellState {
    /// Return the available colors
    #[must_use]
    #[inline]
    pub const fn max_index() -> usize {
        CYCLIC_COLORS.len()
    }
}
