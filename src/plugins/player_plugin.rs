use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
    state::{condition::in_state, state::OnEnter},
};

use crate::{
    states::GameState,
    systems::{
        camera::camera_follow_player,
        physics::{
            collision::player_ground_collision,
            movement::{clamp_velocity, player_input},
        },
        world::{apply_physics, spawn_player},
    },
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    apply_physics,
                    player_input.run_if(in_state(GameState::Playing)),
                    clamp_velocity,
                    player_ground_collision,
                    camera_follow_player,
                )
                    .chain(),
            );
    }
}
