use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
};

use crate::components::{physics::Velocity, tags::Player};

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
