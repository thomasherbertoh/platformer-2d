use bevy::{
    ecs::system::{Res, ResMut},
    input::{ButtonInput, keyboard::KeyCode},
    state::state::NextState,
};

use crate::states::GameState;

pub fn exit_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.pressed(KeyCode::Escape) {
        next_state.set(GameState::Playing);
    }
}
