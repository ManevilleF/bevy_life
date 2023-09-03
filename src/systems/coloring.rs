use crate::{CellState, SimulationBatch};
use bevy::prelude::*;

#[inline]
fn apply_color<S>(state: &S, visible: &mut Visibility, sprite: &mut Sprite)
where
    S: CellState,
{
    match state.color() {
        Some(c) => {
            sprite.color = c;
            if *visible != Visibility::Inherited {
                *visible = Visibility::Inherited;
            }
        }
        None => *visible = Visibility::Hidden,
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn color_sprites<S>(
    mut query: Query<(&S, &mut Visibility, &mut Sprite), Changed<S>>,
    batch: Option<Res<SimulationBatch>>,
) where
    S: CellState,
{
    if batch.is_some() {
        query
            .par_iter_mut()
            .for_each(|(state, mut visible, mut sprite)| {
                apply_color(state, &mut visible, &mut sprite);
            });
    } else {
        for (state, mut visible, mut sprite) in &mut query {
            apply_color(state, &mut visible, &mut sprite);
        }
    }
}
