use crate::resources::materials::CellStateMaterials;
use crate::{CellState, ColorResponse};
use bevy::asset::Asset;
use bevy::ecs::component::Component;
use bevy::prelude::*;

enum ColorOrHandle<A: Asset> {
    Color(Color),
    Handle(Handle<A>),
}

fn color_state<S, A>(
    (state, visible): (&S, &mut Visible),
    cell_materials: &CellStateMaterials<A>,
) -> Option<ColorOrHandle<A>>
where
    S: CellState + Component,
    A: Asset,
{
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
    match &handle {
        None => visible.is_visible = false,
        Some(_m) => {
            visible.is_visible = true;
        }
    }
    handle
}

#[allow(clippy::type_complexity)]
pub fn color_states<S, A>(
    mut query: Query<(&S, &mut Visible, &mut Handle<A>), Changed<S>>,
    cell_materials: Res<CellStateMaterials<A>>,
    mut materials: ResMut<Assets<A>>,
) where
    S: CellState + Component,
    A: Asset + From<Color>,
{
    for (state, mut visible, mut handle) in query.iter_mut() {
        if let Some(h) = color_state((state, &mut visible), &cell_materials) {
            let target_handle = match h {
                ColorOrHandle::Color(c) => materials.add((c).into()),
                ColorOrHandle::Handle(h) => h.clone(),
            };
            *handle = target_handle;
        }
    }
}
