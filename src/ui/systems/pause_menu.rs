use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
};

use crate::{
    game::resources::Config,
    ui::{
        components::{MenuAction, PauseMenuUI},
        systems::menu::{Menu, do_spawn_menu},
    },
};

pub struct PauseMenu;

impl Menu for PauseMenu {
    fn spawn_menu(commands: Commands, asset_server: Res<AssetServer>, config: Res<Config>) {
        do_spawn_menu(
            commands,
            asset_server,
            vec![
                ("Resume", MenuAction::Play),
                ("Main Menu", MenuAction::BackToMainMenu),
            ],
            PauseMenuUI,
            &config.colours,
        );
    }
}
