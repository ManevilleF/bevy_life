//! # Bevy Cellular Automaton
//!
//! [![workflow](https://github.com/ManevilleF/bevy_life/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/bevy_life/actions/workflows/rust.yml)
//!
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![Crates.io](https://img.shields.io/crates/v/bevy_life.svg)](https://crates.io/crates/bevy_life)
//! [![aragog](https://docs.rs/bevy_life/badge.svg)](https://docs.rs/bevy_life)
//! [![dependency status](https://deps.rs/crate/bevy_life/0.1.0/status.svg)](https://deps.rs/crate/bevy_life)
//!
//! `bevy_life` is a generic plugin for [cellular automaton](https://en.wikipedia.org/wiki/Cellular_automaton).
//! From the classic 2D [Conway's game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) to [WireWorld](https://en.wikipedia.org/wiki/Wireworld) and 3D rules, the plugin is completely generic and dynamic.
//!
//! See:
//!  - [Game of life variations](https://cs.stanford.edu/people/eroberts/courses/soco/projects/2008-09/modeling-natural-systems/gameOfLife2.html)
//!  - [Wireworld implementation](https://www.quinapalus.com/wi-index.html)
//!  
//! ## Bevy versions
//!
//! The `main` branch follows the released version of `bevy` (0.5) but I provide 3 useful branches to follow the new engine features:
//! - [bevy_main](https://github.com/ManevilleF/bevy_life/tree/feat/bevy_main) follows the `main` branch of `bevy`
//! - [bevy_pipelined_rendering](https://github.com/ManevilleF/bevy_life/tree/feat/bevy_pipelined_rendering) follows the `pipelined-rendering` branch of `bevy` to use the new rendering system
//! - [sprite_instancing](https://github.com/ManevilleF/bevy_life/tree/feat/sprite_instancing) follows a branch (see [#2642](https://github.com/bevyengine/bevy/pull/2642)) with sprite instacing and batching for better performance.
//!
//! ## How to use
//!
//! You may add as many generic `CellularAutomatonPlugin` as wished, the lib provides some implementations like:
//! - `GameOfLife2dPlugin`
//! - `GameOfLife3dPlugin`
//! - `WireWorld2dPlugin`
//! - `WireWorld3dPlugin`
//! - `CyclicColors2dPlugin`
//! - `CyclicColors3dPlugin`
//!
//! Then you may use bevy as usual and add `impl Cell` and `impl CellState`  components to the entities.
//! The lib provides some implementations like `MooreCell2d` or `MooreCell3d` for cells and `ClassicCellState`, `WireWorldCellState` or `CyclicCellState` for states.
//!
//! You may implement your own cells (coordinate system) and states (rules) as you want, the cellular automaton system is completely dynamic and generic.
//!
//! For more information yo may look at some [examples](./examples).
//!
//! ## Cargo Features
//!
//! No feature is required for the plugin to work and the main traits `Cell` and `CellState` are always available.
//! But you may enable the following features
//!
//! - `2D` (enabled by default): Enables 2D types like:
//!   - `MooreCell2d` (square cell with 8 neighbors)
//!   - `NeumannCell2d` (square cell with 4 neighbors)
//!   - `HexagonCell2d` (hexagon cell with 6 neighbors)
//!   - plugin presets: `GameOfLife2dPlugin`, `WireWorld2dPlugin`, `CyclicAutomaton2dPlugin`
//! - `3D` (enabled by default): Enables 3D types like:
//!   - `MooreCell3d` (cube cell with 26 neighbors)
//!   - `NeumannCell3d` (cube cell with 6 neighbors)
//!   - plugin presets: `GameOfLife3dPlugin`, `WireWorld3dPlugin`, `CyclicAutomaton3dPlugin`
//! - `auto-coloring`:
//!   - Enables `CellStateMaterials2d` (if `2D`) and `CellStateMaterials3d` (if `3D`) types to contain material handles
//!   - The `CellState` type now requires to build either of the previous type (according to 2D/3D feature gates)
//!   - All `CellState` components with materials will be colored according to their type.
//!
//! ## Disclaimer
//!
//! This is probably not the fastest rust implementation of a cellular automaton in rust.
//! For example, using Gosper's [HashLife](https://www.drdobbs.com/jvm/an-algorithm-for-compressing-space-and-t/184406478) a classic game of life could be much faster.
//!
//! This library aim is to be generic and dynamic, so that you can integrate cellular automata to any project in bevy, with any rules, in 2D or 3D.
//!
#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![deny(warnings)]

use bevy::core::FixedTimestep;
use bevy::ecs::component::Component;
use bevy::prelude::*;
use std::marker::PhantomData;

mod components;
mod resources;
mod systems;

use std::fmt::Debug;
pub use {components::*, resources::*};

#[cfg(feature = "2D")]
/// Cellular automaton plugin type for Conway's Game of life in 2D.
pub type GameOfLife2dPlugin = CellularAutomatonPlugin<components::MooreCell2d, ConwayCellState>;

#[cfg(feature = "3D")]
/// Cellular automaton plugin type for Conway's Game of life in 3D.
pub type GameOfLife3dPlugin = CellularAutomatonPlugin<components::MooreCell3d, ConwayCellState>;

#[cfg(feature = "2D")]
/// Cellular automaton plugin type for a binary (blue and orange) Immigration Game of life variation in 2D.
pub type ImmigrationGame2dPlugin =
    CellularAutomatonPlugin<components::MooreCell2d, ImmigrationCellState>;

#[cfg(feature = "3D")]
/// Cellular automaton plugin type for a binary (blue and orange) Immigration Game of life variation in 3D.
pub type ImmigrationGame3dPlugin =
    CellularAutomatonPlugin<components::MooreCell3d, ImmigrationCellState>;

#[cfg(feature = "2D")]
/// Cellular automaton plugin type for a binary (blue and orange) Immigration Game of life variation in 2D.
pub type RainbowGame2dPlugin = CellularAutomatonPlugin<components::MooreCell2d, RainbowCellState>;

#[cfg(feature = "3D")]
/// Cellular automaton plugin type for a binary (blue and orange) Immigration Game of life variation in 3D.
pub type RainbowGame3dPlugin = CellularAutomatonPlugin<components::MooreCell3d, RainbowCellState>;

#[cfg(feature = "2D")]
/// Cellular automaton plugin type for WireWorld in 2D
pub type WireWorld2dPlugin =
    CellularAutomatonPlugin<components::MooreCell2d, components::WireWorldCellState>;

#[cfg(feature = "3D")]
/// Cellular automaton plugin type for WireWorld in 3D
pub type WireWorld3dPlugin =
    CellularAutomatonPlugin<components::MooreCell3d, components::WireWorldCellState>;

#[cfg(feature = "2D")]
/// Cellular automaton plugin type for Colored Cyclic cellular automaton in 2D
pub type CyclicColors2dPlugin =
    CellularAutomatonPlugin<components::MooreCell2d, CyclicColorCellState>;

#[cfg(feature = "3D")]
/// Cellular automaton plugin type for Colored Cyclic cellular automaton in 3D
pub type CyclicColors3dPlugin =
    CellularAutomatonPlugin<components::MooreCell3d, CyclicColorCellState>;

/// Generic Cellular Automaton plugin. It will register systems for the matching `Cell` and `CellState` types.
pub struct CellularAutomatonPlugin<C, S> {
    /// Custom time step constraint value for the systems. If not set, the systems will run every frame.
    pub tick_time_step: Option<f64>,
    /// Phantom data for the `C` (`Cell`) type
    pub phantom_c: PhantomData<C>,
    /// Phantom data for the `S` (`CellState`) type
    pub phantom_s: PhantomData<S>,
}

impl<C: Cell + Component + Debug, S: CellState + Component + Debug> Plugin
    for CellularAutomatonPlugin<C, S>
{
    fn build(&self, app: &mut AppBuilder) {
        let system_set = SystemSet::new()
            .with_system(systems::cells::handle_cells::<C, S>.system().label("cells"))
            .with_system(
                systems::cells::handle_new_cells::<C>
                    .system()
                    .before("cells"),
            );
        let system_set = if let Some(time_step) = self.tick_time_step {
            system_set.with_run_criteria(FixedTimestep::step(time_step))
        } else {
            system_set
        };
        app.add_system_set(system_set);
        app.insert_resource(CellMap::<C>::default());

        #[cfg(feature = "auto-coloring")]
        {
            #[cfg(feature = "2D")]
            {
                app.add_startup_system(Self::setup_materials_2d.system());
                app.add_system(systems::coloring::color_states_2d::<S>.system());
            }
            #[cfg(feature = "3D")]
            {
                app.add_startup_system(Self::setup_materials_3d.system());
                app.add_system(systems::coloring::color_states_3d::<S>.system());
            }
        }
    }
}

impl<C: Cell + Component + Debug, S: CellState + Component + Debug> CellularAutomatonPlugin<C, S> {
    /// Instantiates Self with custom `tick_time_step` value for systems execution
    pub fn new(tick_time_step: f64) -> Self {
        Self {
            tick_time_step: Some(tick_time_step),
            ..Default::default()
        }
    }

    #[cfg(all(feature = "auto-coloring", feature = "2D"))]
    fn setup_materials_2d(
        mut commands: Commands,
        mut assets: ResMut<Assets<bevy::prelude::ColorMaterial>>,
    ) {
        let color_assets = S::setup_materials_2d(&mut assets);
        commands.insert_resource(color_assets);
    }

    #[cfg(all(feature = "auto-coloring", feature = "3D"))]
    fn setup_materials_3d(
        mut commands: Commands,
        mut assets: ResMut<Assets<bevy::prelude::StandardMaterial>>,
    ) {
        let color_assets = S::setup_materials_3d(&mut assets);
        commands.insert_resource(color_assets);
    }
}

impl<C, S> Default for CellularAutomatonPlugin<C, S> {
    fn default() -> Self {
        Self {
            tick_time_step: None,
            phantom_c: Default::default(),
            phantom_s: Default::default(),
        }
    }
}
