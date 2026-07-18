use bevy::{
    ecs::component::Component,
    math::{Vec2, Vec3},
};
use serde::{Deserialize, Serialize};
use shared::components::BlockType;

#[derive(Component)]
pub struct EndGate;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct World;

#[derive(Component)]
pub struct WorldBoundary;

#[derive(Copy, Clone, Component, Debug, Deserialize, Serialize)]
pub struct Block {
    pub pos: Vec3,
    pub size: Vec2,
    pub block_type: BlockType,
}

impl Block {
    pub fn new(pos: Vec3, size: Vec2, block_type: BlockType) -> Self {
        Self {
            pos,
            size,
            block_type,
        }
    }
}
