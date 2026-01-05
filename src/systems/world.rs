use bevy::{
    color::Color,
    ecs::system::{Commands, Query, Res},
    math::Vec2,
    sprite::Sprite,
    time::Time,
    transform::components::{GlobalTransform, Transform},
};

use crate::{
    components::{
        physics::{Collider, Gravity, Velocity},
        tags::{Ground, Player},
    },
    resources::world_bounds::WorldBounds,
};

pub fn apply_physics(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity, &Gravity)>) {
    for (mut transform, mut velocity, gravity) in &mut query {
        velocity.0.y -= gravity.0 * time.delta_secs();
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Velocity(Vec2::ZERO),
        Gravity(512.0),
        Collider {
            size: Vec2::new(10.0, 25.0),
        },
        Sprite {
            color: Color::linear_rgb(1.0, 0.4, 0.5),
            custom_size: Some(Vec2::new(10.0, 25.0)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));
}

pub fn build_floor(mut commands: Commands) {
    let ground_size = Vec2::new(200.0, 10.0);
    commands.spawn((
        Ground,
        Collider { size: ground_size },
        Sprite {
            color: Color::WHITE,
            custom_size: Some(ground_size),
            ..Default::default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
        GlobalTransform::default(),
    ));
    commands.insert_resource(WorldBounds::new(
        Vec2::new(-500.0, -300.0),
        Vec2::new(500.0, 300.0),
    ));
}
