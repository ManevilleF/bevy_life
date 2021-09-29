use crate::components::Cell;
use crate::components::CellState;
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
pub type ClassicGameOfLife2dPlugin = GameOfLifePlugin<components::Cell2d, ClassicCellState>;

#[cfg(feature = "3D")]
pub type ClassicGameOfLife3dPlugin = GameOfLifePlugin<components::Cell3d, ClassicCellState>;

#[cfg(feature = "2D")]
pub type WireWorldGameOfLife2dPlugin =
    GameOfLifePlugin<components::Cell2d, components::WireWorldCellState>;

#[cfg(feature = "3D")]
pub type WireWorldGameOfLife3dPlugin =
    GameOfLifePlugin<components::Cell3d, components::WireWorldCellState>;

#[cfg(feature = "2D")]
pub type CyclicGameOfLife2dPlugin = GameOfLifePlugin<components::Cell2d, CyclicCellState>;

#[cfg(feature = "3D")]
pub type CyclicGameOfLife3dPlugin = GameOfLifePlugin<components::Cell3d, CyclicCellState>;

pub struct GameOfLifePlugin<C, S> {
    pub tick_time_step: Option<f64>,
    pub phantom_c: PhantomData<C>,
    pub phantom_s: PhantomData<S>,
}

impl<C: Cell + Component + Debug, S: CellState + Component + Debug> Plugin
    for GameOfLifePlugin<C, S>
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

impl<C: Cell + Component + Debug, S: CellState + Component + Debug> GameOfLifePlugin<C, S> {
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

impl<C, S> Default for GameOfLifePlugin<C, S> {
    fn default() -> Self {
        Self {
            tick_time_step: None,
            phantom_c: Default::default(),
            phantom_s: Default::default(),
        }
    }
}
