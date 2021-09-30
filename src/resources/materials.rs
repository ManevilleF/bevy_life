#![allow(dead_code)]

use bevy::prelude::*;

#[cfg(feature = "2D")]
#[derive(Debug)]
/// Global material handle container ressource for 2D
pub struct CellStateMaterials2d {
    /// material handles vector
    pub materials: Vec<Handle<bevy::prelude::ColorMaterial>>,
}

#[cfg(feature = "3D")]
#[derive(Debug)]
/// Global material handle container ressource for 3D
pub struct CellStateMaterials3d {
    /// material handles vector
    pub materials: Vec<Handle<bevy::prelude::StandardMaterial>>,
}
