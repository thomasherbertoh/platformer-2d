use bevy::{
    ecs::component::Component,
    math::{Vec2, Vec3},
};
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct SplashEntity;

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct OnGround(pub bool);

#[derive(Component)]
pub struct FootSensor;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum BlockType {
    #[default]
    Floor,
    PlayerSpawn,
    End,
}

#[derive(Clone, Component, Debug, Deserialize, Serialize)]
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
