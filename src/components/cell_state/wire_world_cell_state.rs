use crate::components::CellState;
#[cfg(feature = "auto-coloring")]
use crate::ColorResponse;
#[cfg(feature = "auto-coloring")]
use bevy::prelude::Color;
use bevy::prelude::Component;

/// Wireworld is a cellular automaton that simulates electronic devices and logic gates by having cells represent electrons traveling across conductors.
/// Wireworld uses three possible cell states and has the following rules:
///
/// - Electron heads (`ElectronHead`) become electron tails in the succeeding generation.
/// - Electron tails (`ElectronTail`) become conductors.
/// - Conductors (`Conductor`) become electron heads if exactly one or two neighboring cells are electron heads. Otherwise, they remain as conductors.
#[derive(Clone, Debug, PartialEq, Component)]
pub enum WireWorldCellState {
    /// Conductor cell state
    Conductor,
    /// Electron head cell state
    ElectronHead,
    /// Electron tail cell state
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
    fn color_or_material_index(&self) -> ColorResponse {
        ColorResponse::MaterialIndex(match self {
            WireWorldCellState::Conductor => 0,
            WireWorldCellState::ElectronHead => 1,
            WireWorldCellState::ElectronTail => 2,
        })
    }

    #[cfg(feature = "auto-coloring")]
    fn colors() -> &'static [Color] {
        &[Color::GOLD, Color::CYAN, Color::WHITE]
    }
}
