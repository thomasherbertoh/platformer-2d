use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
};
use bevy_rapier2d::prelude::Velocity;

use crate::player::components::{OnGround, Player};

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let speed = 200.0;

    for mut velocity in &mut query {
        let mut move_dir = 0.0;
        if keyboard.pressed(KeyCode::KeyA) {
            move_dir -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            move_dir += 1.0;
        }

        velocity.linvel.x = move_dir * speed;
    }
}

pub fn jump_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &OnGround), With<Player>>,
) {
    let jump_velocity = 400.0;

    for (mut velocity, on_ground) in &mut query {
        if on_ground.0
            && (keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::KeyW))
        {
            velocity.linvel.y = jump_velocity;
        }
    }
}
