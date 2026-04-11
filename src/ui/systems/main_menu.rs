use bevy::{
    asset::AssetServer,
    ecs::{
        message::MessageReader,
        query::With,
        system::{Commands, Query, Res},
    },
    ui::widget::Text,
};

use crate::{
    game::{events::LoadLevelEvent, resources::Config},
    ui::{
        components::{MainMenuUI, MenuAction},
        resources::HeadingText,
        systems::menu::{Menu, do_spawn_menu},
    },
};

pub struct MainMenu;

impl Menu for MainMenu {
    fn spawn_menu(commands: Commands, asset_server: Res<AssetServer>, config: Res<Config>) {
        do_spawn_menu(
            commands,
            asset_server,
            vec![
                ("Play", MenuAction::Play),
                ("Level Select", MenuAction::LevelSelect),
                ("Quit", MenuAction::Quit),
            ],
            MainMenuUI,
            &config.colours,
            "Main Menu - No Level Selected".to_string(),
        );
    }
}

pub fn update_level_name_in_main_menu_heading(
    mut reader: MessageReader<LoadLevelEvent>,
    mut query: Query<&mut Text, (With<HeadingText>, With<MainMenuUI>)>,
) {
    if let Some(load_level_event) = reader.read().next()
        && let Ok(mut text) = query.single_mut()
    {
        text.0 = format!(
            "Main Menu - {}",
            load_level_event
                .path
                .split("/")
                .last()
                .map(|s| s.trim_end_matches(".json"))
                .unwrap()
        )
    }
}
