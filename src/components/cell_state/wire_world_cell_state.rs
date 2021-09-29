use crate::components::CellState;
#[cfg(feature = "auto-coloring")]
use bevy::prelude::{Assets, Color};

/// Wireworld is a cellular automaton that simulates electronic devices and logic gates by having cells represent electrons traveling across conductors.
/// Wireworld uses three possible cell states and has the following rules:
///
/// - Electron heads (`ElectronHead`) become electron tails in the succeeding generation.
/// - Electron tails (`ElectronTail`) become conductors.
/// - Conductors (`Conductor`) become electron heads if exactly one or two neighboring cells are electron heads. Otherwise, they remain as conductors.
#[derive(Clone, Debug, PartialEq)]
pub enum WireWorldCellState {
    Conductor,
    ElectronHead,
    ElectronTail,
}

impl Default for WireWorldCellState {
    fn default() -> Self {
        Self::Conductor
    }
}

impl CellState for WireWorldCellState {
    fn new_cell_state(&self, neighbor_cells: &[Self]) -> Self {
        match self {
            Self::Conductor => {
                let electron_head_count = neighbor_cells
                    .iter()
                    .filter(|&c| matches!(c, Self::ElectronHead))
                    .count();
                if (1..=2).contains(&electron_head_count) {
                    Self::ElectronHead
                } else {
                    Self::Conductor
                }
            }
            Self::ElectronHead => Self::ElectronTail,
            Self::ElectronTail => Self::Conductor,
        }
    }

    #[cfg(feature = "auto-coloring")]
    fn material_index(&self) -> usize {
        match self {
            WireWorldCellState::Conductor => 0,
            WireWorldCellState::ElectronHead => 1,
            WireWorldCellState::ElectronTail => 2,
        }
    }

    #[cfg(all(feature = "auto-coloring", feature = "2D"))]
    fn setup_materials_2d(
        materials: &mut Assets<bevy::prelude::ColorMaterial>,
    ) -> crate::resources::materials::CellStateMaterials2d {
        crate::resources::materials::CellStateMaterials2d {
            materials: vec![
                materials.add(Color::GOLD.into()),
                materials.add(Color::CYAN.into()),
                materials.add(Color::WHITE.into()),
            ],
        }
    }

    #[cfg(all(feature = "auto-coloring", feature = "3D"))]
    fn setup_materials_3d(
        materials: &mut Assets<bevy::prelude::StandardMaterial>,
    ) -> crate::resources::materials::CellStateMaterials3d {
        crate::resources::materials::CellStateMaterials3d {
            materials: vec![
                materials.add(Color::GOLD.into()),
                materials.add(Color::CYAN.into()),
                materials.add(Color::WHITE.into()),
            ],
        }
    }
}
