use std::fs;

use bevy::{
    ecs::system::{Res, ResMut},
    input::{ButtonInput, keyboard::KeyCode},
    state::state::NextState,
};

use crate::game::{
    resources::Config,
    states::{GameState, MenuState},
};

pub fn load_config() -> Config {
    let config_str = fs::read_to_string("configs/config.json")
        .expect("Failed to read config file: `configs/config.json`");
    serde_json::from_str(&config_str).expect("Failed to parse config file: `configs/config.json`")
}

pub fn pause_game(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
        menu_state.set(MenuState::Paused);
    }
}
