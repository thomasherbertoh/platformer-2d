use bevy::ecs::resource::Resource;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerDimensions {
    pub height: f32,
    pub width: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerMovement {
    pub accel: f32,
    pub move_velocity: f32,
    pub jump_velocity: f32,
}

#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct Config {
    pub player_dimensions: PlayerDimensions,
    pub player_movement: PlayerMovement,
}
