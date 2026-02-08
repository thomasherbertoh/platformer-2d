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
    states::{GameState, MenuState},
    ui::{
        components::{GameOverMenuUI, MenuUI},
        systems::{
            game_over_menu::spawn_game_over_menu,
            main_menu::spawn_main_menu,
            menu::{despawn_with, menu_button_system},
        },
    },
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MenuState>()
            .add_systems(OnEnter(MenuState::Main), spawn_main_menu)
            .add_systems(OnExit(MenuState::Main), despawn_with::<MenuUI>)
            .add_systems(OnEnter(MenuState::GameOver), spawn_game_over_menu)
            .add_systems(OnExit(MenuState::GameOver), despawn_with::<GameOverMenuUI>)
            .add_systems(Update, menu_button_system.run_if(in_state(GameState::Menu)));
    }
}
