use bevy::{
    color::Color,
    ecs::system::Commands,
    math::Vec2,
    sprite::Sprite,
    transform::components::{GlobalTransform, Transform},
};
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, GravityScale, LockedAxes, RigidBody, Sensor, Velocity,
};

use crate::{
    components::tags::{FootSensor, Ground, OnGround, Player},
    resources::world_bounds::WorldBounds,
};

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            Player,
            OnGround(false),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            GravityScale(1.0),
            Collider::cuboid(5.0, 12.5),
            ActiveEvents::COLLISION_EVENTS,
            Sprite {
                color: Color::linear_rgb(1.0, 0.4, 0.5),
                custom_size: Some(Vec2::new(10.0, 25.0)),
                ..Default::default()
            },
            Transform::from_xyz(0.0, 50.0, 0.0),
            GlobalTransform::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                FootSensor,
                Sensor,
                Collider::cuboid(4.5, 1.0),
                ActiveEvents::COLLISION_EVENTS,
                Transform::from_xyz(0.0, -13.5, 0.0),
                GlobalTransform::default(),
            ));
        });
}

pub fn build_floor(mut commands: Commands) {
    let ground_size = Vec2::new(400.0, 10.0);
    commands.spawn((
        Ground,
        Collider::cuboid(ground_size.x / 2.0, ground_size.y / 2.0),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(ground_size),
            ..Default::default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
        GlobalTransform::default(),
    ));
    commands.insert_resource(WorldBounds {
        min: Vec2::new(-500.0, -300.0),
        max: Vec2::new(500.0, 300.0),
    });
}
