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
    player::components::{CoyoteTime, JumpBuffer, OnGround, Player},
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
        
        velocity.linear.x = move_dir * config.player_movement.move_velocity;
    }
}

pub fn jump_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    config: Res<Config>,
    mut query: Query<(&mut Velocity, &OnGround, &mut CoyoteTime, &mut JumpBuffer), With<Player>>,
) {
    for (mut velocity, on_ground, mut coyote, mut buffer) in &mut query {
        let jump_key_pressed =
            keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::KeyW);
        // buffer a jump every time the user presses jump
        if jump_key_pressed {
            buffer.timer.reset();
        }

        /*
         * We want to jump if:
         * - we've just pressed jump and we're on the ground
         * - we've just pressed jump and we've just walked off an edge (coyote time)
         * - we're on the ground and we pressed jump recently (jump buffer)
         */
        if ((on_ground.0 || !coyote.timer.is_finished()) && jump_key_pressed)
            || (on_ground.0 && !buffer.timer.is_finished())
        {
            // Jump!
            velocity.linear.y = config.player_movement.jump_velocity;

            // prevent double-jump
            coyote.consume();
            buffer.consume();
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

pub fn update_jump_buffer(time: Res<Time>, mut query: Query<&mut JumpBuffer>) {
    for mut buffer in query.iter_mut() {
        buffer.timer.tick(time.delta());
    }
}
