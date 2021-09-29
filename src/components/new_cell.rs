use crate::cell::Cell;

#[cfg(feature = "2D")]
pub type NewCell2d = NewCell<crate::cell::Cell2d>;
#[cfg(feature = "3D")]
pub type NewCell3d = NewCell<crate::cell::Cell3d>;

#[derive(Debug)]
pub struct NewCell<T: Cell>(pub T);

#[cfg(feature = "2D")]
impl NewCell2d {
    pub fn new(coords: bevy::math::IVec2) -> Self {
        Self(crate::cell::Cell2d { coords })
    }
}

#[cfg(feature = "3D")]
impl NewCell3d {
    pub fn new(coords: bevy::math::IVec3) -> Self {
        Self(crate::cell::Cell3d { coords })
    }
}
