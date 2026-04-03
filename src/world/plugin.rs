use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::IntoScheduleConfigs,
    state::{condition::in_state, state::OnEnter},
};

use crate::{
    camera::systems::{
        center_camera_on_world, spawn_world_camera, update_camera_projection_on_resize,
    },
    game::states::GameState,
    world::systems::{build_world, setup_level},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_level)
            .add_systems(
                OnEnter(GameState::Playing),
                (build_world, spawn_world_camera),
            )
            .add_systems(
                Update,
                ((center_camera_on_world, update_camera_projection_on_resize)
                    .run_if(in_state(GameState::Playing)),),
            );
    }
}
