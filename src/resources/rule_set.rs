/// A set of rules for the game of life
#[derive(Debug, Clone)]
pub struct RuleSet {
    /// Any live cell with fewer than `underpopulation_threshold` live neighbours dies (referred to as underpopulation or exposure)
    pub underpopulation_threshold: u8,
    /// Any live cell with more than `overpopulation_threshold` live neighbours dies (referred to as overpopulation or overcrowding)
    pub overpopulation_threshold: u8,
    /// Any dead cell with exactly `reproduction_value` live neighbours will come to life.
    ///
    /// You can set multiple values
    pub reproduction_value: Vec<u8>,
}

impl Default for RuleSet {
    fn default() -> Self {
        Self {
            underpopulation_threshold: 2,
            overpopulation_threshold: 3,
            reproduction_value: vec![3],
        }
    }
}
