use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
};

use crate::ui::{
    components::{MainMenuUI, MenuAction},
    systems::menu::{Menu, do_spawn_menu},
};

pub struct MainMenu;

impl Menu for MainMenu {
    fn spawn_menu(commands: Commands, asset_server: Res<AssetServer>) {
        do_spawn_menu(
            commands,
            asset_server,
            vec![("Play", MenuAction::Play), ("Quit", MenuAction::Quit)],
            MainMenuUI,
        );
    }
}
