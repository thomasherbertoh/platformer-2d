use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
};
use bevy_rapier2d::prelude::Velocity;

use crate::{
    game::resources::Config,
    player::components::{OnGround, Player},
};

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    config: Res<Config>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in &mut query {
        let mut move_dir = 0.0;
        if keyboard.pressed(KeyCode::KeyA) {
            move_dir -= config.player_movement.accel;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            move_dir += config.player_movement.accel;
        }

        velocity.linvel.x = move_dir * config.player_movement.move_velocity;
    }
}

pub fn jump_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    config: Res<Config>,
    mut query: Query<(&mut Velocity, &OnGround), With<Player>>,
) {
    for (mut velocity, on_ground) in &mut query {
        if on_ground.0
            && (keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::KeyW))
        {
            velocity.linvel.y = config.player_movement.jump_velocity;
        }
    }
}
