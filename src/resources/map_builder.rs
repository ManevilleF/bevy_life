use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub struct MapBuilder {
    pub custom_seed: Option<String>,
    pub cell_padding: f32,
}

impl MapBuilder {
    pub fn seed(&self) -> String {
        self.custom_seed.as_ref().map_or_else(
            || {
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(32)
                    .map(char::from)
                    .collect()
            },
            |s| s.clone(),
        )
    }
}

impl Default for MapBuilder {
    fn default() -> Self {
        Self {
            custom_seed: None,
            cell_padding: 0.0,
        }
    }
}
