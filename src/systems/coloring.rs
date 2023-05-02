use crate::CellState;
use bevy::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn color_sprites<S>(
    mut query: Query<(&S, &mut Visibility, &mut Sprite), Changed<S>>,
    // batch: Option<Res<SimulationBatch>>,
) where
    S: CellState,
{
    let apply_color =
        |state: &S, visible: &mut Visibility, mut sprite: &mut Sprite| match state.color() {
            Some(c) => {
                sprite.color = c;
                *visible = Visibility::Inherited;
            }
            None => *visible = Visibility::Hidden,
        };

    // Enabe this code on 0.10.1 (See https://github.com/bevyengine/bevy/pull/8029)
    //
    // if batch.is_some() {
    //     query
    //         .par_iter_mut()
    //         .for_each_mut(|(state, mut visible, mut sprite)| {
    //             apply_color(state, &mut visible, &mut sprite);
    //         });
    // } else {
    for (state, mut visible, mut sprite) in query.iter_mut() {
        apply_color(state, &mut visible, &mut sprite);
    }
    // }
}
