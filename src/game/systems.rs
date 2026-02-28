use bevy::{
    ecs::system::{Res, ResMut},
    input::{ButtonInput, keyboard::KeyCode},
    state::state::NextState,
};

use crate::game::states::{GameState, MenuState};

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
