#![allow(dead_code)]

use bevy::prelude::*;

#[cfg(feature = "2D")]
#[derive(Debug)]
pub struct CellStateMaterials2d {
    pub materials: Vec<Handle<bevy::prelude::ColorMaterial>>,
}

#[cfg(feature = "3D")]
#[derive(Debug)]
pub struct CellStateMaterials3d {
    pub materials: Vec<Handle<bevy::prelude::StandardMaterial>>,
}
