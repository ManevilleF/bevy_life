//! `bevy_life` is a generic plugin for [cellular automaton](https://en.wikipedia.org/wiki/Cellular_automaton).
//! From the classic 2D [Conway's game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) to [`WireWorld`](https://en.wikipedia.org/wiki/Wireworld) and 3D rules, the plugin is completely generic and dynamic.
//!
//! See:
//!
//! * [Game of life variations](https://cs.stanford.edu/people/eroberts/courses/soco/projects/2008-09/modeling-natural-systems/gameOfLife2.html)
//! * [`Wireworld` implementation](https://www.quinapalus.com/wi-index.html) (see
//!   this lib's [implementation](https://github.com/ManevilleF/wireworld-rs))
//!  
//! ## Bevy versions
//!
//! The `main` branch follows the released version of `bevy` but I provide the [`bevy-main`](https://github.com/ManevilleF/bevy_life/tree/feat/bevy-main) branch
//! to follow the `main` branch of `bevy`
//!
//! | `bevy_life`   | `bevy`    |
//! |---------------|-----------|
//! | 0.3.x         | 0.6.x     |
//! | 0.4.x         | 0.7.x     |
//! | 0.5.x         | 0.8.x     |
//! | 0.6.x         | 0.9.x     |
//! | 0.7.x         | 0.10.x    |
//! | 0.8.x         | 0.11.x    |
//! | 0.9.x         | 0.13.x    |
//! | 0.10.x        | 0.14.x    |
//!
//! ## How to use
//!
//! Add a `CellularAutomatonPlugin` to your bevy app:
//!
//! A `CellularAutomatonPlugin<C, S>` has two generic types:
//!
//! * `C` -> Any type implementing `Cell`, defining the coordinate system
//! * `S` -> Any type implementing `CellState`, defining the simulation rules.
//!
//! You may add as many generic `CellularAutomatonPlugin` as wished, the lib
//! provides some implementations like:
//!
//! * `GameOfLife2dPlugin`
//! * `GameOfLife3dPlugin`
//! * `ImmigrationGame2dPlugin`
//! * `ImmigrationGame3dPlugin`
//! * `RainbowGame2dPlugin`
//! * `RainbowGame3dPlugin`
//! * `WireWorld2dPlugin`
//! * `WireWorld3dPlugin`
//! * `CyclicColors2dPlugin`
//! * `CyclicColors3dPlugin`
//!
//! Then you may use bevy as usual and add `impl Cell` and `impl CellState`
//! components to the entities. The lib provides some implementations like
//! `MooreCell2d` or `MooreCell3d` for cells and `ConwayCellState`,
//! `WireWorldCellState`, etc for states.
//!
//! You may implement your own *cells* (coordinate system) and *states* (rules)
//! as you want, the cellular automaton system is completely dynamic and
//! generic.
//!
//! For more information you may look at some examples:
//!
//! * The [Classic examples](./examples) showcase the provided implementations
//! * the [Rock Paper Scissor](./examples/2d_rock_paper_scissor.rs) defines
//!   custom rules.
//! * the [wireworld](https://github.com/ManevilleF/wireworld-rs) repository
//!
//! ### Pausing
//!
//! Inserting a `SimulationPause` resource will pause the simulation, removing
//! it wil resume the it.
//!
//! ### Parallel execution and batching
//!
//! Inserting a `SimulationBatch` resource will allow parallel computation of
//! cells with custom batch sizes.
//!
//! ## Cargo Features
//!
//! No feature is required for the plugin to work and the main traits `Cell` and
//! `CellState` are always available. But you may enable the following features
//!
//! * `2D` (enabled by default): Enables 2D types like:
//!   * `MooreCell2d` (square cell with 8 neighbors)
//!   * `NeumannCell2d` (square cell with 4 neighbors)
//!   * `HexagonCell2d` (hexagon cell with 6 neighbors)
//!   * plugin presets: `GameOfLife2dPlugin`, `ImmigrationGame2dPlugin`,
//!     `RainbowGame2dPlugin`, `WireWorld2dPlugin`, `CyclicAutomaton2dPlugin`
//! * `3D`: Enables 3D types like:
//!   * `MooreCell3d` (cube cell with 26 neighbors)
//!   * `NeumannCell3d` (cube cell with 6 neighbors)
//!   * plugin presets: `GameOfLife3dPlugin`, `ImmigrationGame3dPlugin`,
//!     `RainbowGame3dPlugin`, `WireWorld3dPlugin`, `CyclicAutomaton3dPlugin`
//! * `auto-coloring` (Example or debug purpose):
//!   * The `CellState` trait now requires a `color` method
//! * `bevy_reflect` (enabled by default): Enable support for reflection for
//!   common types
//!
//! ## Disclaimer
//!
//! This is probably not the fastest rust implementation of a cellular automaton
//! in rust. For example, using Gosper's [`HashLife`](https://www.drdobbs.com/jvm/an-algorithm-for-compressing-space-and-t/184406478) a classic game of life could be much faster.
//!
//! This library aim is to be generic and dynamic, so that you can integrate
//! cellular automata to any project in bevy, with any rules, in 2D or 3D.
#![forbid(missing_docs, unsafe_code)]
#![warn(
    clippy::nursery,
    clippy::pedantic,
    nonstandard_style,
    rustdoc::broken_intra_doc_links
)]
#![allow(clippy::default_trait_access, clippy::module_name_repetitions)]

use bevy::{log, prelude::*, time::common_conditions::on_timer};
use std::{marker::PhantomData, time::Duration};

mod components;
mod resources;
mod systems;

use systems::cells::{handle_cells, handle_new_cells};

use crate::systems::cells::handle_removed_cells;
pub use components::*;
pub use resources::*;

#[cfg(feature = "2D")]
/// Cellular automaton plugin type for Conway's Game of life in 2D.
pub type GameOfLife2dPlugin = CellularAutomatonPlugin<components::MooreCell2d, ConwayCellState>;

#[cfg(feature = "3D")]
/// Cellular automaton plugin type for Conway's Game of life in 3D.
pub type GameOfLife3dPlugin = CellularAutomatonPlugin<components::MooreCell3d, ConwayCell4555State>;

#[cfg(feature = "2D")]
/// Cellular automaton plugin type for a binary (blue and orange) Immigration
/// Game of life variation in 2D.
pub type ImmigrationGame2dPlugin =
    CellularAutomatonPlugin<components::MooreCell2d, ImmigrationCellState>;

#[cfg(feature = "3D")]
/// Cellular automaton plugin type for a binary (blue and orange) Immigration
/// Game of life variation in 3D.
pub type ImmigrationGame3dPlugin =
    CellularAutomatonPlugin<components::MooreCell3d, ImmigrationCellState>;

#[cfg(feature = "2D")]
/// Cellular automaton plugin type for a binary (blue and orange) Immigration
/// Game of life variation in 2D.
pub type RainbowGame2dPlugin = CellularAutomatonPlugin<components::MooreCell2d, RainbowCellState>;

#[cfg(feature = "3D")]
/// Cellular automaton plugin type for a binary (blue and orange) Immigration
/// Game of life variation in 3D.
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
pub type CyclicColors2dPlugin<const N: usize> =
    CellularAutomatonPlugin<components::MooreCell2d, CyclicColorCellState<N>>;

#[cfg(feature = "3D")]
/// Cellular automaton plugin type for Colored Cyclic cellular automaton in 3D
pub type CyclicColors3dPlugin<const N: usize> =
    CellularAutomatonPlugin<components::MooreCell3d, CyclicColorCellState<N>>;

/// System set variant for each cellular automaton step
#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
pub enum LifeSystemSet {
    /// Spawned Cell insertion in [`CellMap`] (Requires
    /// [`CellularAutomatonPlugin::use_cell_map`])
    NewCells,
    /// Despawned Cell removal from [`CellMap`] (Requires
    /// [`CellularAutomatonPlugin::use_cell_map`])
    RemovedCells,
    /// Cell life tick update system set
    CellUpdate,
}

/// Generic Cellular Automaton plugin. It will register systems for the matching
/// `Cell` and `CellState` types.
///
/// The `BATCH_SIZE` const argument determines the size of query batches to be
/// queried in parallel. It has a big performance impact on worlds with a lot of
/// cells.
pub struct CellularAutomatonPlugin<C, S> {
    /// Custom time step (in seconds) constraint value for the systems. If not
    /// set, the systems will run every frame.
    pub tick_time_step: Option<f64>,
    /// Should a [`CellMap`] be resource be added and filled ?
    pub use_cell_map: bool,
    /// Phantom data for the `C` (`Cell`) type
    pub phantom_c: PhantomData<C>,
    /// Phantom data for the `S` (`CellState`) type
    pub phantom_s: PhantomData<S>,
}

impl<C: Cell, S: CellState> Plugin for CellularAutomatonPlugin<C, S> {
    fn build(&self, app: &mut App) {
        // app.register_type::<C>().register_type::<S>().
        // register_type::<CellMap::<C>>();
        if self.use_cell_map {
            app.insert_resource(CellMap::<C>::default());
            app.add_systems(
                PostUpdate,
                (
                    handle_new_cells::<C>.in_set(LifeSystemSet::NewCells),
                    handle_removed_cells::<C>.in_set(LifeSystemSet::RemovedCells),
                ),
            );
        }
        if let Some(time_step) = self.tick_time_step {
            let duration = Duration::from_secs_f64(time_step);
            app.add_systems(
                Update,
                handle_cells::<C, S>
                    .run_if(on_timer(duration))
                    .in_set(LifeSystemSet::CellUpdate),
            );
        } else {
            app.add_systems(
                Update,
                handle_cells::<C, S>.in_set(LifeSystemSet::CellUpdate),
            );
        }

        #[cfg(feature = "auto-coloring")]
        {
            #[cfg(feature = "2D")]
            {
                app.add_systems(Update, systems::coloring::color_sprites::<S>);
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
    /// Instantiates Self with default values
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            tick_time_step: None,
            use_cell_map: false,
            phantom_c: PhantomData,
            phantom_s: PhantomData,
        }
    }

    /// Sets a custom `tick_time_step` value for systems execution
    #[must_use]
    #[inline]
    pub const fn with_time_step(mut self, tick_time_step: f64) -> Self {
        self.tick_time_step = Some(tick_time_step);
        self
    }

    /// The plugin will set a [`CellMap`] resource and dynamically update it
    #[must_use]
    #[inline]
    pub const fn with_cell_map(mut self) -> Self {
        self.use_cell_map = true;
        self
    }
}

impl<C, S> Default for CellularAutomatonPlugin<C, S> {
    fn default() -> Self {
        Self::new()
    }
}
