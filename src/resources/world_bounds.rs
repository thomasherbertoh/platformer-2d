use bevy::{ecs::resource::Resource, math::Vec2};

#[derive(Resource)]
pub struct WorldBounds {
    pub min: Vec2,
    pub max: Vec2,
}
