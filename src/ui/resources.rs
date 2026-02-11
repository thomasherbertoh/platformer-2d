use bevy::{ecs::resource::Resource, time::Timer};

use crate::game::states::{GameState, MenuState};

#[derive(Resource)]
pub struct GameTimer {
    pub timer: Timer,
    pub target_state: GameState,
    pub next_game_state: GameState,
    pub next_menu_state: MenuState,
}
