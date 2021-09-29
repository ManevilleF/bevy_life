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

#[derive(Debug, Clone)]
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
            .map(|c| materials.add(c.clone().into()))
            .collect();
        crate::materials::CellStateMaterials2d { materials }
    }

    #[cfg(all(feature = "auto-coloring", feature = "3D"))]
    fn setup_materials_3d(
        materials: &mut Assets<bevy::prelude::StandardMaterial>,
    ) -> crate::materials::CellStateMaterials3d {
        let materials = CYCLIC_COLORS
            .iter()
            .map(|c| materials.add(c.clone().into()))
            .collect();
        crate::materials::CellStateMaterials3d { materials }
    }
}

impl CyclicCellState {
    pub fn pos(&self) -> usize {
        CYCLIC_COLORS.iter().position(|&c| c == self.0).unwrap_or(0)
    }

    pub const fn available_colors() -> &'static [Color; 9] {
        &CYCLIC_COLORS
    }
}

impl Default for CyclicCellState {
    fn default() -> Self {
        Self(CYCLIC_COLORS[0].clone())
    }
}
