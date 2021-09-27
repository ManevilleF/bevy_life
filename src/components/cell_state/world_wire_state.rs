use crate::components::CellState;

/// Wireworld is a cellular automaton that simulates electronic devices and logic gates by having cells represent electrons traveling across conductors.
/// Wireworld uses three possible cell states and has the following rules:
/// -Electron heads (`ElectronHead`) become electron tails in the succeeding generation.
/// -Electron tails (`ElectronTail`) become conductors.
/// -Conductors (`Conductor`) become electron heads if exactly one or two neighboring cells are electron heads. Otherwise, they remain as conductors.
#[derive(Clone, Debug)]
pub enum WorldWireCellState {
    Conductor,
    ElectronHead,
    ElectronTail,
}

impl CellState for WorldWireCellState {
    fn new_cell_state(&self, neighbor_cells: &[&Self]) -> Self {
        match self {
            Self::Conductor => {
                let electron_head_count = neighbor_cells
                    .iter()
                    .filter(|c| matches!(c, Self::ElectronHead))
                    .count();
                if electron_head_count == 1 || electron_head_count == 2 {
                    Self::ElectronHead
                } else {
                    Self::Conductor
                }
            }
            Self::ElectronHead => Self::ElectronTail,
            Self::ElectronTail => Self::Conductor,
        }
    }
}
