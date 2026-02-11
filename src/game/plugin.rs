use bevy::{
    app::{App, Plugin, Update},
    state::{
        app::AppExtStates,
        state::{OnEnter, OnExit},
    },
};

use crate::{
    game::states::GameState,
    player::{plugin::PlayerPlugin, resources::GroundContacts},
    ui::{
        plugins::menu::MenuPlugin,
        systems::{
            splash::{cleanup_splash, setup_splash},
            timer::tick_game_timer,
            win::{cleanup_win, win_screen_system},
        },
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
            .add_systems(Update, tick_game_timer);
    }
}
