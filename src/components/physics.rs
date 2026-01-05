use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct MaxVelocity(pub Vec2);

#[derive(Component, Default)]
pub struct Gravity(pub f32);

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}
