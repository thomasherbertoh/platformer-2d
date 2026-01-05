use bevy::{ecs::resource::Resource, math::Vec2};

#[derive(Resource)]
pub struct WorldBounds {
    min: Vec2,
    max: Vec2,
}

impl WorldBounds {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn get_min(&self) -> Vec2 {
        self.min
    }

    pub fn get_max(&self) -> Vec2 {
        self.max
    }
}
