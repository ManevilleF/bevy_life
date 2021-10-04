# Changelog

## Unreleased

- `CellMap`:
  - Using a `bevy::utils::Hashmap` instead of the standard library one, which is slightly faster
- Cells system:
  - Removed reference couting `Arc` from the `RwLock` for the batched query iterations
- Using batched queries for the coloring systems (`auto-coloring` feature)
- Examples:
  - Improved examples map sizes
  - Reworked the color system for the provided `CellStates`

## 0.2.0

### Changed (**BREAKING CHANGES**)

- `3D` feature gate is no longer enabled by default
- Complete rework of the `auto-coloring` features, these changes are not detailed as this feature is provided for example purposes.

Renamed:
  - `Cell2d` -> `MooreCell2d`
  - `Cell3d` -> `MooreCell3d`
  - `ClassicCellState` -> `ConwayCellState`

### Added

- New `Cell` implementations:
  - `NeumannCell2d` (4 neighbors)
  - `NeumannCell3d` (6 neighbors)
  - `HexagonCell2d` (6 neighbors in cubic space)
- New `CellState` implementations:
  - `Conway4555CellState` for game of life *4555* rules
  - `ImmigrationCellState` for the immigration game (bi color)
  - `RainbowCellState` for the rainbow game (gray scale)

The `CellularAutomatonPlugin` now takes an additional `BATCH_SIZE: usize` const parameter defining the new query batch size for better parallel execution.
The system handling cells and states now uses parallel querying with this new parameter

New examples are added.

## 0.1.0

Initial version
