use bevy::prelude::Resource;

/// Resource to insert for parallel queries and batching
#[derive(Debug, Copy, Clone, Resource, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct SimulationBatch;

/// Resource to insert to pause the cellular automaton simulation
#[derive(Debug, Copy, Clone, Resource, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct SimulationPause;
