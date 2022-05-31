//! # Bevy Cellular Automaton
//!
//! [![workflow](https://github.com/ManevilleF/bevy_life/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/bevy_life/actions/workflows/rust.yml)
//!
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![Crates.io](https://img.shields.io/crates/v/bevy_life.svg)](https://crates.io/crates/bevy_life)
//! [![Docs.rs](https://docs.rs/bevy_life/badge.svg)](https://docs.rs/bevy_life)
//! [![dependency status](https://deps.rs/crate/bevy_life/0.4.0/status.svg)](https://deps.rs/crate/bevy_life)
//!
//! `bevy_life` is a generic plugin for [cellular automaton](https://en.wikipedia.org/wiki/Cellular_automaton).
//! From the classic 2D [Conway's game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) to [`WireWorld`](https://en.wikipedia.org/wiki/Wireworld) and 3D rules, the plugin is completely generic and dynamic.
//!
//! See:
//!  - [Game of life variations](https://cs.stanford.edu/people/eroberts/courses/soco/projects/2008-09/modeling-natural-systems/gameOfLife2.html)
//!  - [`Wireworld` implementation](https://www.quinapalus.com/wi-index.html)
//!  
//! ## Bevy versions
//!
//! The `main` branch follows the released version of `bevy` (0.6) but I provide the [`bevy-main`](https://github.com/ManevilleF/bevy_life/tree/feat/bevy-main) branch
//! to follow the `main` branch of `bevy`
//!
//! ## How to use
//!
//! You may add as many generic `CellularAutomatonPlugin` as wished, the lib provides some implementations like:
//! - `GameOfLife2dPlugin`
//! - `GameOfLife3dPlugin`
//! - `ImmigrationGame2dPlugin`
//! - `ImmigrationGame3dPlugin`
//! - `RainbowGame2dPlugin`
//! - `RainbowGame3dPlugin`
//! - `WireWorld2dPlugin`
//! - `WireWorld3dPlugin`
//! - `CyclicColors2dPlugin`
//! - `CyclicColors3dPlugin`
//!
//! Then you may use bevy as usual and add `impl Cell` and `impl CellState`  components to the entities.
//! The lib provides some implementations like `MooreCell2d` or `MooreCell3d` for cells and `ConwayCellState`, `WireWorldCellState`, etc for states.
//!
//! You may implement your own *cells* (coordinate system) and *states* (rules) as you want, the cellular automaton system is completely dynamic and generic.
//!
//! For more information yo may look at some [examples](./examples).
//!
//! ### Pausing
//!
//! Inserting a `SimulationPause` resource will pause the simulation, removing it wil resume the it.
//!
//! ### Parallel execution and batching
//!
//! Inserting a `SimulationBatch` resource will allow parallel computation of cells with custom batch sizes.
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
//!   - plugin presets: `GameOfLife2dPlugin`, `ImmigrationGame2dPlugin`, `RainbowGame2dPlugin`, `WireWorld2dPlugin`, `CyclicAutomaton2dPlugin`
//! - `3D`: Enables 3D types like:
//!   - `MooreCell3d` (cube cell with 26 neighbors)
//!   - `NeumannCell3d` (cube cell with 6 neighbors)
//!   - plugin presets: `GameOfLife3dPlugin`, `ImmigrationGame3dPlugin`, `RainbowGame3dPlugin`, `WireWorld3dPlugin`, `CyclicAutomaton3dPlugin`
//! - `auto-coloring` (Example or debug purpose):
//!   - Enables `CellStateMaterials` resource to contain material handles
//!   - The `CellState` type now requires to build a `CellStateMaterials`
//!   - All `CellState` components with materials will be colored according to their type.
//!
//! ## Disclaimer
//!
//! This is probably not the fastest rust implementation of a cellular automaton in rust.
//! For example, using Gosper's [`HashLife`](https://www.drdobbs.com/jvm/an-algorithm-for-compressing-space-and-t/184406478) a classic game of life could be much faster.
//!
//! This library aim is to be generic and dynamic, so that you can integrate cellular automata to any project in bevy, with any rules, in 2D or 3D.
//!
#![forbid(missing_docs, unsafe_code)]
#![warn(
    clippy::nursery,
    clippy::pedantic,
    nonstandard_style,
    rustdoc::broken_intra_links
)]
#![allow(clippy::default_trait_access, clippy::module_name_repetitions)]

use bevy::log;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use std::marker::PhantomData;

mod components;
mod resources;
mod systems;

pub use {components::*, resources::*};

#[cfg(feature = "2D")]
/// Cellular automaton plugin type for Conway's Game of life in 2D.
pub type GameOfLife2dPlugin = CellularAutomatonPlugin<components::MooreCell2d, ConwayCellState>;

#[cfg(feature = "3D")]
/// Cellular automaton plugin type for Conway's Game of life in 3D.
pub type GameOfLife3dPlugin = CellularAutomatonPlugin<components::MooreCell3d, ConwayCell4555State>;

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
/// Cellular automaton plugin type for `WireWorld` in 2D
pub type WireWorld2dPlugin =
    CellularAutomatonPlugin<components::MooreCell2d, components::WireWorldCellState>;

#[cfg(feature = "3D")]
/// Cellular automaton plugin type for `WireWorld` in 3D
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
///
/// The `BATCH_SIZE` const argument determines the size of query batches to be queried in parallel.
/// It has a big performance impact on worlds with a lot of cells.
pub struct CellularAutomatonPlugin<C, S> {
    /// Custom time step constraint value for the systems. If not set, the systems will run every frame.
    pub tick_time_step: Option<f64>,
    /// Phantom data for the `C` (`Cell`) type
    pub phantom_c: PhantomData<C>,
    /// Phantom data for the `S` (`CellState`) type
    pub phantom_s: PhantomData<S>,
}

impl<C: Cell, S: CellState> Plugin for CellularAutomatonPlugin<C, S> {
    fn build(&self, app: &mut App) {
        let system_set = SystemSet::new()
            .with_system(systems::cells::handle_cells::<C, S>.label("cells"))
            .with_system(systems::cells::handle_new_cells::<C>.before("cells"));
        let system_set = if let Some(time_step) = self.tick_time_step {
            system_set.with_run_criteria(FixedTimestep::step(time_step))
        } else {
            system_set
        };
        app.add_system_set(system_set);
        app.insert_resource(CellMap::<C>::default());
        app.register_type::<C>().register_type::<S>();

        #[cfg(feature = "auto-coloring")]
        {
            #[cfg(feature = "2D")]
            {
                app.add_system(systems::coloring::color_sprites::<S>);
            }
            #[cfg(feature = "3D")]
            {
                log::warn!("No auto coloring is available for 3D materials");
            }
        }
        log::info!("Loaded cellular automaton plugin");
    }
}

impl<C, S> CellularAutomatonPlugin<C, S> {
    /// Instantiates Self with custom `tick_time_step` value for systems execution
    #[must_use]
    #[inline]
    pub fn new(tick_time_step: f64) -> Self {
        Self {
            tick_time_step: Some(tick_time_step),
            ..Default::default()
        }
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
