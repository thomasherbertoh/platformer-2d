use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
};

use crate::components::{
    physics::{MaxVelocity, Velocity},
    tags::Player,
};

pub fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.single_mut() {
        velocity.0.x = 0.0;
        if keyboard.pressed(KeyCode::KeyA) {
            velocity.0.x = -200.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            velocity.0.x = 200.0;
        }
    }
}

pub fn clamp_velocity(mut query: Query<(&mut Velocity, &MaxVelocity)>) {
    for (mut velocity, max) in &mut query {
        velocity.0.x = velocity.0.x.clamp(-max.0.x, max.0.x);
        velocity.0.y = velocity.0.y.clamp(-max.0.y, max.0.y);
    }
}
