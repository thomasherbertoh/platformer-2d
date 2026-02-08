use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
    state::condition::in_state,
};

use crate::{
    camera::systems::camera_follow_player,
    game::states::GameState,
    player::systems::{
        collision::{
            clamp_velocity_when_grounded, end_gate_collision_system, foot_sensor_collision_system,
            ground_detection_system, update_grounded_state, world_boundary_collision_system,
        },
        movement::{jump_system, player_movement},
    },
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    player_movement,
                    jump_system,
                    end_gate_collision_system,
                    world_boundary_collision_system,
                )
                    .run_if(in_state(GameState::Playing)),
                ground_detection_system,
                foot_sensor_collision_system,
                update_grounded_state,
                clamp_velocity_when_grounded,
                camera_follow_player,
            )
                .chain(),
        );
    }
}
