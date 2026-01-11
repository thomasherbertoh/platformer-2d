use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
    state::{condition::in_state, state::OnEnter},
};

use crate::{
    states::GameState,
    systems::{
        camera::{center_camera_on_world, spawn_camera, update_camera_projection_on_resize},
        world::build_floor,
    },
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (build_floor, spawn_camera))
            .add_systems(
                Update,
                ((center_camera_on_world, update_camera_projection_on_resize)
                    .run_if(in_state(GameState::Playing)),),
            );
    }
}
