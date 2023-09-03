# Changelog

## [Unreleased]

* Added rustfmt config (#19)

## 0.8.0

* Added automatic `CellMap` update on `Cell` component removal
* Native `CellState` code cleanup
* (**BREAKING**) `CellMap` insertion and dynamic update is now optional and disabled
by default
* Performance improvements
* (**BREAKING**) `CellState::new_cell_state` now takes an iterator instead of a slice
* Bump `bevy` to 0.11

## 0.7.0

* `bevy` 0.10
* (**BREAKING**) `SimulationBatch` is now a unit struct
* (**BREAKING**) Renamed `CellularAutomatonPlugin::new` to `with_time_step`
* Temporarily disabled batching for the `auto-coloring` systems, until this
[bug](https://github.com/bevyengine/bevy/pull/8029) is adressed

## 0.6.0

* `bevy` 0.9
* Performance improvements for batched simulations

## 0.5.0

* `bevy` 0.8
* Dropped `lazy_static` implementation
* Improved `README`
* Added a rock paper scissor example
* Eased the trait bounds on `Cell` and `CellState`

## 0.4.0

* `bevy` 0.7

## 0.3.1

* Clippy `pedantic`
* Added `must_use` and `inline` attributes
* All components implement `Reflect` and are registered to the app
* Added example declaration

## 0.3.0

* Bevy 0.6
* Rust 2021
* Clippy extra restrictions

## 0.2.3

* Removed `BATCH_SIZE` generic constant value from `CellularAutomatonPlugin`
* Added `SimulationBatch` resource to handle parallel computation and batches
* Auto coloring job optimized
* Fixed example windows
* Removed `2d_wireworld` example. A better standalone version is available [here](https://github.com/ManevilleF/wireworld-rs)

## 0.2.2

* Added `CellMap<T>::get_cell` method to retrieve a cell entity
* Added a `SimulationPause` resource to allow simulation pausing
* Improved logging in systems
* `CellMap<T>::insert_cell` now returns the previus entity if it was present

## 0.2.1

* `CellMap`:
  * Using a `bevy::utils::Hashmap` instead of the standard library one, which
  is slightly faster
* Cells system:
  * Removed reference counting `Arc` from the `RwLock` for the batched query iterations
* Using batched queries for the coloring systems (`auto-coloring` feature)
* Examples:
  * Improved examples map sizes
  * Reworked the color system for the provided `CellStates`
* Issues Github template

## 0.2.0

### Changed (**BREAKING CHANGES**)

* `3D` feature gate is no longer enabled by default
* Complete rework of the `auto-coloring` features, these changes are not detailed
as this feature is provided for example purposes.

Renamed:

* `Cell2d` -> `MooreCell2d`
* `Cell3d` -> `MooreCell3d`
* `ClassicCellState` -> `ConwayCellState`

### Added

* New `Cell` implementations:
  * `NeumannCell2d` (4 neighbors)
  * `NeumannCell3d` (6 neighbors)
  * `HexagonCell2d` (6 neighbors in cubic space)
* New `CellState` implementations:
  * `Conway4555CellState` for game of life *4555* rules
  * `ImmigrationCellState` for the immigration game (bi color)
  * `RainbowCellState` for the rainbow game (gray scale)

The `CellularAutomatonPlugin` now takes an additional `BATCH_SIZE: usize` const
parameter defining the new query batch size for better parallel execution.
The system handling cells and states now uses parallel querying with this new parameter

New examples are added.

## 0.1.0

Initial version
