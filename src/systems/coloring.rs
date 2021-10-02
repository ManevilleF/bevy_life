use crate::resources::materials::CellStateMaterials;
use crate::{CellState, ColorResponse};
use bevy::asset::Asset;
use bevy::ecs::component::Component;
use bevy::prelude::*;

#[allow(clippy::type_complexity)]
pub fn color_states<S, A>(
    mut commands: Commands,
    mut query: Query<(Entity, &S, &mut Visible), Changed<S>>,
    cell_materials: Res<CellStateMaterials<A>>,
    mut materials: ResMut<Assets<A>>,
) where
    S: CellState + Component,
    A: Asset + From<Color>,
{
    for (entity, state, mut visible) in query.iter_mut() {
        let response: ColorResponse = state.color_or_material_index();
        let handle = match response {
            ColorResponse::MaterialIndex(i) => cell_materials.materials.get(i).cloned(),
            ColorResponse::Color(c) => Some(materials.add(c.into())),
            ColorResponse::None => None,
        };
        match handle {
            None => visible.is_visible = false,
            Some(m) => {
                visible.is_visible = true;
                commands.entity(entity).insert(m);
            }
        };
    }
}
