/// Resource to insert for parallel queries and batching
#[derive(Debug, Clone)]
pub struct SimulationBatch {
    /// Batch size for parallel execution
    pub batch_size: usize,
}

impl Default for SimulationBatch {
    fn default() -> Self {
        Self { batch_size: 1000 }
    }
}
