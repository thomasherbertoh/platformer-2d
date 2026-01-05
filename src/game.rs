use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
    state::{app::AppExtStates, condition::in_state},
};

use crate::{
    plugins::{player_plugin::PlayerPlugin, world_plugin::WorldPlugin},
    states::GameState,
    systems::management::exit_menu,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::default())
            .add_plugins((PlayerPlugin, WorldPlugin))
            .add_systems(Update, exit_menu.run_if(in_state(GameState::MainMenu)));
    }
}
