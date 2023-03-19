use bevy::prelude::Resource;

/// Resource to insert for parallel queries and batching
#[derive(Debug, Clone, Resource, Default)]
pub struct SimulationBatch;

/// Resource to insert to pause the cellular automaton simulation
#[derive(Debug, Resource)]
pub struct SimulationPause;
