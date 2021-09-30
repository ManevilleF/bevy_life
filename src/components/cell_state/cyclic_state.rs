use crate::CellState;
#[cfg(feature = "auto-coloring")]
use bevy::asset::Assets;
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
pub struct CyclicCellState(pub Color);

impl CellState for CyclicCellState {
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
    fn material_index(&self) -> usize {
        self.pos()
    }

    #[cfg(all(feature = "auto-coloring", feature = "2D"))]
    fn setup_materials_2d(
        materials: &mut Assets<bevy::prelude::ColorMaterial>,
    ) -> crate::materials::CellStateMaterials2d {
        let materials = CYCLIC_COLORS
            .iter()
            .map(|c| materials.add((*c).into()))
            .collect();
        crate::materials::CellStateMaterials2d { materials }
    }

    #[cfg(all(feature = "auto-coloring", feature = "3D"))]
    fn setup_materials_3d(
        materials: &mut Assets<bevy::prelude::StandardMaterial>,
    ) -> crate::materials::CellStateMaterials3d {
        let materials = CYCLIC_COLORS
            .iter()
            .map(|c| materials.add((*c).into()))
            .collect();
        crate::materials::CellStateMaterials3d { materials }
    }
}

impl CyclicCellState {
    /// The index of `self` in the used `CYCLIC_COLORS` const color array
    pub fn pos(&self) -> usize {
        CYCLIC_COLORS.iter().position(|&c| c == self.0).unwrap_or(0)
    }

    /// Return the available colors
    pub const fn available_colors() -> &'static [Color; 9] {
        &CYCLIC_COLORS
    }
}

impl Default for CyclicCellState {
    fn default() -> Self {
        Self(CYCLIC_COLORS[0])
    }
}
