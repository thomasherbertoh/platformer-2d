use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
};

use crate::ui::{
    components::{GameOverMenuUI, MenuAction},
    systems::menu::{Menu, do_spawn_menu},
};

pub struct GameOverMenu;

impl Menu for GameOverMenu {
    fn spawn_menu(commands: Commands, asset_server: Res<AssetServer>) {
        do_spawn_menu(
            commands,
            asset_server,
            vec![
                ("Retry", MenuAction::Play),
                ("Main Menu", MenuAction::BackToMainMenu),
            ],
            GameOverMenuUI,
        );
    }
}
