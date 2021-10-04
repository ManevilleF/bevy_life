use crate::resources::materials::CellStateMaterials;
use crate::{CellState, ColorResponse};
use bevy::asset::Asset;
use bevy::ecs::component::Component;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use std::sync::RwLock;

enum ColorOrHandle<A: Asset> {
    Color(Color),
    Handle(Handle<A>),
}

#[allow(clippy::type_complexity)]
pub fn color_states<S, A, const BATCH_SIZE: usize>(
    mut commands: Commands,
    pool: Res<ComputeTaskPool>,
    mut query: Query<(Entity, &S, &mut Visible), Changed<S>>,
    cell_materials: Res<CellStateMaterials<A>>,
    mut materials: ResMut<Assets<A>>,
) where
    S: CellState + Component,
    A: Asset + From<Color>,
{
    let vec = RwLock::new(Vec::new());
    query.par_for_each_mut(&pool, BATCH_SIZE, |(entity, state, mut visible)| {
        let response: ColorResponse = state.color_or_material_index();
        let handle = match response {
            ColorResponse::MaterialIndex(i) => cell_materials
                .materials
                .get(i)
                .cloned()
                .map(ColorOrHandle::Handle),
            ColorResponse::Color(c) => Some(ColorOrHandle::Color(c)),
            ColorResponse::None => None,
        };
        match handle {
            None => visible.is_visible = false,
            Some(m) => {
                visible.is_visible = true;
                let mut lock = vec.write().unwrap();
                lock.push((entity, m));
            }
        };
    });
    let lock = vec.read().unwrap();
    for (e, m) in lock.iter() {
        let handle = match m {
            ColorOrHandle::Color(c) => materials.add((*c).into()),
            ColorOrHandle::Handle(h) => h.clone(),
        };
        commands.entity(*e).insert(handle);
    }
}
