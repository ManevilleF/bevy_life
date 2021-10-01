#[cfg(feature = "2D")]
use crate::resources::materials::CellStateMaterials2d;
#[cfg(feature = "3D")]
use crate::resources::materials::CellStateMaterials3d;
use crate::{CellState, ColorResponse};
use bevy::ecs::component::Component;
use bevy::log;
use bevy::prelude::*;

#[cfg(feature = "2D")]
#[allow(clippy::type_complexity)]
pub fn color_states_2d<S>(
    mut commands: Commands,
    query: Query<(Entity, &S), (With<Handle<ColorMaterial>>, Changed<S>)>,
    cell_materials: Res<CellStateMaterials2d>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) where
    S: CellState + Component,
{
    for (entity, state) in query.iter() {
        let response: ColorResponse = state.color_or_material_index();
        let handle = match response {
            ColorResponse::MaterialIndex(i) => cell_materials.materials.get(i).cloned(),
            ColorResponse::Color(c) => Some(materials.add(c.into())),
        };
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
    cell_materials: Res<CellStateMaterials3d>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) where
    S: CellState + Component,
{
    for (entity, state) in query.iter() {
        let response: ColorResponse = state.color_or_material_index();
        let handle = match response {
            ColorResponse::MaterialIndex(i) => cell_materials.materials.get(i).cloned(),
            ColorResponse::Color(c) => Some(materials.add(c.into())),
        };
        if let Some(material) = handle {
            commands.entity(entity).insert(material);
        } else {
            log::error!("No material found for cell state {:?}", state);
        }
    }
}
