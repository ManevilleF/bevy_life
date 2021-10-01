use crate::resources::materials::CellStateMaterials;
use crate::{CellState, ColorResponse};
use bevy::asset::Asset;
use bevy::ecs::component::Component;
use bevy::log;
use bevy::prelude::*;

#[allow(clippy::type_complexity)]
pub fn color_states<S, A>(
    mut commands: Commands,
    query: Query<(Entity, &S), (With<Handle<A>>, Changed<S>)>,
    cell_materials: Res<CellStateMaterials<A>>,
    mut materials: ResMut<Assets<A>>,
) where
    S: CellState + Component,
    A: Asset + From<Color>,
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
