use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ArenaConfig {
    pub width: f32,
    pub height: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        ArenaConfig {
            width: 256.0,
            height: 224.0
        }
    }
}