use crate::{CellState, SimulationBatch};
use bevy::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn color_sprites<S>(
    mut query: Query<(&S, &mut Visibility, &mut Sprite), Changed<S>>,
    batch: Option<Res<SimulationBatch>>,
) where
    S: CellState,
{
    let apply_color =
        |state: &S, mut visible: &mut Visibility, mut sprite: &mut Sprite| match state.color() {
            Some(c) => {
                sprite.color = c;
                visible.is_visible = true;
            }
            None => visible.is_visible = false,
        };

    if let Some(config) = batch {
        query.par_for_each_mut(config.batch_size, |(state, mut visible, mut sprite)| {
            apply_color(state, &mut visible, &mut sprite);
        });
    } else {
        for (state, mut visible, mut sprite) in query.iter_mut() {
            apply_color(state, &mut visible, &mut sprite);
        }
    }
}
