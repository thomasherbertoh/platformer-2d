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
    game::states::{GameState, MenuState},
    ui::{
        components::{GameOverMenuUI, MainMenuUI},
        systems::{
            button::menu_button_system,
            game_over_menu::GameOverMenu,
            main_menu::MainMenu,
            menu::{Menu, despawn_with},
        },
    },
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MenuState>()
            .add_systems(OnEnter(MenuState::Main), <MainMenu as Menu>::spawn_menu)
            .add_systems(OnExit(MenuState::Main), despawn_with::<MainMenuUI>)
            .add_systems(
                OnEnter(MenuState::GameOver),
                <GameOverMenu as Menu>::spawn_menu,
            )
            .add_systems(OnExit(MenuState::GameOver), despawn_with::<GameOverMenuUI>)
            .add_systems(Update, menu_button_system.run_if(in_state(GameState::Menu)));
    }
}
