use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
    time::Time,
};
use bevy_rapier2d::prelude::Velocity;

use crate::{
    game::resources::Config,
    player::components::{CoyoteTime, OnGround, Player},
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
    mut query: Query<(&mut Velocity, &OnGround, &mut CoyoteTime), With<Player>>,
) {
    for (mut velocity, on_ground, mut coyote) in &mut query {
        if (on_ground.0 || !coyote.timer.is_finished())
            && (keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::KeyW))
        {
            velocity.linvel.y = config.player_movement.jump_velocity;

            // prevent double-jump
            coyote.consume();
        }
    }
}

pub fn update_coyote_time(time: Res<Time>, mut query: Query<(&mut CoyoteTime, &OnGround)>) {
    for (mut coyote, grounded) in query.iter_mut() {
        if grounded.0 {
            coyote.timer.reset();
        } else {
            coyote.timer.tick(time.delta());
        }
    }
}
