use bevy::ecs::resource::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerDimensions {
    pub height: f32,
    pub width: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerMovement {
    accel: f32,
    move_velocity: f32,
    jump_velocity: f32,
}

#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct Config {
    player_dimensions: PlayerDimensions,
    player_movement: PlayerMovement,
}
