use crate::components::BlockData;
use bevy_math::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct WorldBounds {
    pub min: Vec2,
    pub max: Vec2,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LevelData {
    pub blocks: Vec<BlockData>,
    pub world_bounds: WorldBounds,
}
