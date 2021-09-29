#[cfg(feature = "2D")]
use crate::resources::materials::CellStateMaterials2d;
#[cfg(feature = "3D")]
use crate::resources::materials::CellStateMaterials3d;
use crate::CellState;
use bevy::ecs::component::Component;
use bevy::log;
use bevy::prelude::*;

#[cfg(feature = "2D")]
#[allow(clippy::type_complexity)]
pub fn color_states_2d<S>(
    mut commands: Commands,
    query: Query<(Entity, &S), (With<Handle<ColorMaterial>>, Changed<S>)>,
    materials: Res<CellStateMaterials2d>,
) where
    S: CellState + Component,
{
    for (entity, state) in query.iter() {
        let mat_index = state.material_index();
        let handle = materials.materials.get(mat_index).cloned();
        if let Some(material) = handle {
            commands.entity(entity).insert(material);
        } else {
            log::error!("No material found for cell state {:?}", state);
        }
    }
}

#[cfg(feature = "3D")]
#[allow(clippy::type_complexity)]
pub fn color_states_3d<S>(
    mut commands: Commands,
    query: Query<(Entity, &S), (With<Handle<StandardMaterial>>, Changed<S>)>,
    materials: Res<CellStateMaterials3d>,
) where
    S: CellState + Component,
{
    for (entity, state) in query.iter() {
        let mat_index = state.material_index();
        let handle = materials.materials.get(mat_index).cloned();
        if let Some(material) = handle {
            commands.entity(entity).insert(material);
        } else {
            log::error!("No material found for cell state {:?}", state);
        }
    }
}
