use crate::world::components::Block;
use bevy::ecs::resource::Resource;
use serde::{Deserialize, Serialize};
use shared::resources::WorldBounds;

#[derive(Clone, Deserialize, Resource, Serialize)]
pub struct Level {
    pub blocks: Vec<Block>,
    pub world_bounds: WorldBounds,
}

#[derive(Clone, Resource)]
pub struct LevelFile {
    pub path: String,
}
