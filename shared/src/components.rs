use bevy_math::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum BlockType {
    #[default]
    Floor,
    PlayerSpawn,
    End,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockData {
    pub pos: Vec3,
    pub size: Vec2,
    pub block_type: BlockType,
}
