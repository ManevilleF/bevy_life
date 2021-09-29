#![allow(dead_code)]

use bevy::prelude::*;
use bevy::sprite::ColorMaterial;

#[derive(Debug)]
pub struct CellStateMaterials {
    pub materials: Vec<Handle<ColorMaterial>>,
}
