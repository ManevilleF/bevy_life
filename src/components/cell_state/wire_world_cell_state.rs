use crate::components::CellState;
use bevy::prelude::{Component, Reflect};
#[cfg(feature = "auto-coloring")]
use bevy::render::color::Color;

/// Wireworld is a cellular automaton that simulates electronic devices and logic gates by having cells represent electrons traveling across conductors.
/// Wireworld uses three possible cell states and has the following rules:
///
/// - Electron heads (`ElectronHead`) become electron tails in the succeeding generation.
/// - Electron tails (`ElectronTail`) become conductors.
/// - Conductors (`Conductor`) become electron heads if exactly one or two neighboring cells are electron heads. Otherwise, they remain as conductors.
#[derive(Clone, Debug, PartialEq, Component, Reflect)]
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
    fn color(&self) -> Option<Color> {
        Some(match self {
            WireWorldCellState::Conductor => Color::GOLD,
            WireWorldCellState::ElectronHead => Color::CYAN,
            WireWorldCellState::ElectronTail => Color::WHITE,
        })
    }
}
