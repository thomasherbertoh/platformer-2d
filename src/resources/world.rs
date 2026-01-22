use bevy::{ecs::resource::Resource, math::Vec2};
use serde::{Deserialize, Serialize};

use crate::components::tags::Block;

#[derive(Resource)]
pub struct WorldBounds {
    pub min: Vec2,
    pub max: Vec2,
}

#[derive(Clone, Deserialize, Resource, Serialize)]
pub struct LevelData {
    pub blocks: Vec<Block>,
}
