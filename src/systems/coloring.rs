use crate::resources::materials::CellStateMaterials;
use crate::CellState;
use bevy::ecs::component::Component;
use bevy::log;
use bevy::prelude::*;

#[allow(clippy::type_complexity)]
pub fn color_states<S>(
    mut commands: Commands,
    query: Query<(Entity, &S), (With<Handle<ColorMaterial>>, Changed<S>)>,
    materials: Res<CellStateMaterials>,
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
