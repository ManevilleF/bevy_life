<!-- cargo-sync-readme start -->

# Bevy Cellular Automaton

[![workflow](https://github.com/ManevilleF/bevy_life/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/bevy_life/actions/workflows/rust.yml)

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/bevy_life.svg)](https://crates.io/crates/bevy_life)
[![aragog](https://docs.rs/bevy_life/badge.svg)](https://docs.rs/bevy_life)
[![dependency status](https://deps.rs/crate/bevy_life/0.1.0/status.svg)](https://deps.rs/crate/bevy_life)

`bevy_life` is a generic plugin for [cellular automaton](https://en.wikipedia.org/wiki/Cellular_automaton).
From the classic 2D [Conway's game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) to [WireWorld](https://en.wikipedia.org/wiki/Wireworld) and 3D rules, the plugin is completely generic and dynamic.

See:
 - [Game of life variations](https://cs.stanford.edu/people/eroberts/courses/soco/projects/2008-09/modeling-natural-systems/gameOfLife2.html)
 - [Wireworld implementation](https://www.quinapalus.com/wi-index.html)
 
## Bevy versions

The `main` branch follows the released version of `bevy` (0.5) but I provide 3 useful branches to follow the new engine features:
- [bevy_main](https://github.com/ManevilleF/bevy_life/tree/feat/bevy_main) follows the `main` branch of `bevy`
- [bevy_pipelined_rendering](https://github.com/ManevilleF/bevy_life/tree/feat/bevy_pipelined_rendering) follows the `pipelined-rendering` branch of `bevy` to use the new rendering system
- [sprite_instancing](https://github.com/ManevilleF/bevy_life/tree/feat/sprite_instancing) follows a branch (see [#2642](https://github.com/bevyengine/bevy/pull/2642)) with sprite instacing and batching for better performance.

## How to use

You may add as many generic `CellularAutomatonPlugin` as wished, the lib provides some implementations like:
- `GameOfLife2dPlugin`
- `GameOfLife3dPlugin`
- `WireWorld2dPlugin`
- `WireWorld3dPlugin`
- `CyclicColors2dPlugin`
- `CyclicColors3dPlugin`

Then you may use bevy as usual and add `impl Cell` and `impl CellState`  components to the entities.
The lib provides some implementations like `MooreCell2d` or `MooreCell3d` for cells and `ClassicCellState`, `WireWorldCellState` or `CyclicCellState` for states.

You may implement your own cells (coordinate system) and states (rules) as you want, the cellular automaton system is completely dynamic and generic.

For more information yo may look at some [examples](./examples).

## Cargo Features

No feature is required for the plugin to work and the main traits `Cell` and `CellState` are always available.
But you may enable the following features

- `2D` (enabled by default): Enables 2D types like:
  - `MooreCell2d` (square cell with 8 neighbors)
  - `NeumannCell2d` (square cell with 4 neighbors)
  - `HexagonCell2d` (hexagon cell with 6 neighbors)
  - plugin presets: `GameOfLife2dPlugin`, `WireWorld2dPlugin`, `CyclicAutomaton2dPlugin`
- `3D` (enabled by default): Enables 3D types like:
  - `MooreCell3d` (cube cell with 26 neighbors)
  - `NeumannCell3d` (cube cell with 6 neighbors)
  - plugin presets: `GameOfLife3dPlugin`, `WireWorld3dPlugin`, `CyclicAutomaton3dPlugin`
- `auto-coloring`:
  - Enables `CellStateMaterials2d` (if `2D`) and `CellStateMaterials3d` (if `3D`) types to contain material handles
  - The `CellState` type now requires to build either of the previous type (according to 2D/3D feature gates)
  - All `CellState` components with materials will be colored according to their type.


<!-- cargo-sync-readme end -->

## Examples

For every example pressing space reloads the board

### Classic 2D

Run `cargo run --example 2d_classic --features auto-coloring --release`

![Alt](./docs/2d_classic_demo.gif "classic demo gif")

### Cyclic 2D

Run `cargo run --example 2d_cyclic --features auto-coloring --release`

![Alt](./docs/2d_cyclic_demo.gif "cyclic demo gif")

### Wire World 2D

Run `cargo run --example 2d_wireworld --features auto-coloring --release`

The example is dynamic, use the left mouse click to create a conductor cell on an empty space or to create an electron head

![Alt](./docs/2d_wireworld_demo.gif "wireworld demo gif")

![Alt](./docs/2d_wireworld_flip_flop_demo.gif "wireworld flip flop gate gif")