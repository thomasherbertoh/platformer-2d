use bevy::ecs::resource::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Dimensions {
    pub player_height: f32,
    pub player_width: f32,
    pub end_gate_scale_factor: f32,
    pub height_foot_sensor_scale_factor: f32,
    pub width_foot_sensor_scale_factor: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerMovement {
    pub accel: f32,
    pub move_velocity: f32,
    pub jump_velocity: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Colours {
    pub player_colour: Rgb,
    pub level_end_colour: Rgba,
    pub button_background_colour: Rgb,
    pub menu_outer_background_colour: Rgb,
    pub menu_inner_background_colour: Rgb,
}

#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct Config {
    pub dimensions: Dimensions,
    pub player_movement: PlayerMovement,
    pub colours: Colours,
}
