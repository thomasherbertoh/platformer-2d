use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
    state::{
        app::AppExtStates,
        condition::in_state,
        state::{OnEnter, OnExit},
    },
};

use crate::{
    plugins::{menu_plugin::MenuPlugin, player_plugin::PlayerPlugin},
    resources::ground_contacts::GroundContacts,
    states::GameState,
    systems::ui::{
        splash::{cleanup_splash, setup_splash, splash_timer},
        win::{cleanup_win, win_screen_system, win_timer},
    },
    world::{plugin::WorldPlugin, systems::cleanup_world},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::default())
            .init_resource::<GroundContacts>()
            .add_plugins((MenuPlugin, PlayerPlugin, WorldPlugin))
            .add_systems(OnEnter(GameState::SplashScreen), setup_splash)
            .add_systems(OnExit(GameState::SplashScreen), cleanup_splash)
            .add_systems(OnExit(GameState::Playing), cleanup_world)
            .add_systems(OnEnter(GameState::Win), win_screen_system)
            .add_systems(OnExit(GameState::Win), cleanup_win)
            .add_systems(
                Update,
                (
                    splash_timer.run_if(in_state(GameState::SplashScreen)),
                    win_timer.run_if(in_state(GameState::Win)),
                ),
            );
    }
}
