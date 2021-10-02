use crate::CellState;
#[cfg(feature = "auto-coloring")]
use crate::ColorResponse;
use bevy::prelude::Color;

const CYCLIC_COLORS: [Color; 9] = [
    Color::BLUE,
    Color::CYAN,
    Color::GREEN,
    Color::LIME_GREEN,
    Color::YELLOW,
    Color::ORANGE,
    Color::ORANGE_RED,
    Color::RED,
    Color::PURPLE,
];

/// Basic cyclic cellular automaton state and rules. The rules are the following:
///
/// > As with any cellular automaton, the cyclic cellular automaton consists of a regular grid of cells in one or more dimensions.
/// > The cells can take on any of `n` states, ranging from `0`to `n − 1`. The first generation starts out with random states in each of the cells.
/// > In each subsequent generation, if a cell has a neighboring cell whose value is the successor of the cell's value, the cell is *consumed* and takes on the succeeding value.
/// > (Note that `0` is the successor of `n − 1`.
///
/// For this type we use `9` for `n` and arbitrary colors.
#[derive(Debug, Clone, PartialEq)]
pub struct CyclicColorCellState(pub Color);

impl CellState for CyclicColorCellState {
    fn new_cell_state(&self, neighbor_cells: &[Self]) -> Self {
        let pos = self.pos();
        let target_color = if pos >= 8 {
            CYCLIC_COLORS[0]
        } else {
            CYCLIC_COLORS[pos + 1]
        };
        for neighbor_cell in neighbor_cells.iter() {
            if neighbor_cell.0 == target_color {
                return neighbor_cell.clone();
            }
        }
        self.clone()
    }

    #[cfg(feature = "auto-coloring")]
    fn color_or_material_index(&self) -> ColorResponse {
        ColorResponse::MaterialIndex(self.pos())
    }

    #[cfg(feature = "auto-coloring")]
    fn colors() -> &'static [Color] {
        &CYCLIC_COLORS
    }
}

impl CyclicColorCellState {
    /// The index of `self` in the used `CYCLIC_COLORS` const color array
    pub fn pos(&self) -> usize {
        CYCLIC_COLORS.iter().position(|&c| c == self.0).unwrap_or(0)
    }

    /// Return the available colors
    pub const fn available_colors() -> &'static [Color; 9] {
        &CYCLIC_COLORS
    }
}

impl Default for CyclicColorCellState {
    fn default() -> Self {
        Self(CYCLIC_COLORS[0])
    }
}
