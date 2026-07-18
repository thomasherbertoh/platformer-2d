use bevy::{
    ecs::system::{Res, ResMut},
    state::state::{NextState, State},
    time::Time,
};

use crate::{
    game::states::{GameState, MenuState},
    ui::resources::GameTimer,
};

pub fn tick_game_timer(
    time: Res<Time>,
    mut game_timer: ResMut<GameTimer>,
    current_state: Res<State<GameState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    if !game_timer.target_state.eq(current_state.get()) {
        return;
    }

    game_timer.timer.tick(time.delta());
    if game_timer.timer.is_finished() {
        game_state.set(game_timer.next_game_state.clone());
        menu_state.set(game_timer.next_menu_state.clone());
    }
}
