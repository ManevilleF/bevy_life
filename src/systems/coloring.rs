use crate::CellState;
use bevy::prelude::{Changed, Query, Sprite, Visibility};

#[allow(clippy::needless_pass_by_value)]
pub fn color_sprites<S>(mut query: Query<(&S, &mut Visibility, &mut Sprite), Changed<S>>)
where
    S: CellState,
{
    for (state, mut visible, mut sprite) in query.iter_mut() {
        match state.color() {
            Some(c) => {
                sprite.color = c;
                visible.is_visible = true;
            }
            None => visible.is_visible = false,
        }
    }
}
