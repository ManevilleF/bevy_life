#![allow(dead_code)]

use bevy::asset::Asset;
use bevy::prelude::*;

#[derive(Debug)]
/// Global material handle container ressource
pub struct CellStateMaterials<A: Asset> {
    /// material handles vector
    pub materials: Vec<Handle<A>>,
}
