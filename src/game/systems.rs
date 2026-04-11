use std::fs;

use bevy::{
    ecs::{
        message::{MessageReader, MessageWriter},
        system::{Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    log::info,
    state::state::NextState,
};
use rfd::FileDialog;

use crate::game::{
    events::{LoadLevelEvent, OpenFileDialogEvent},
    resources::Config,
    states::{GameState, MenuState},
};

pub fn load_config() -> Config {
    let config_str = fs::read_to_string("configs/config.json")
        .expect("Failed to read config file: `configs/config.json`");
    serde_json::from_str(&config_str).expect("Failed to parse config file: `configs/config.json`")
}

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

pub fn open_file_dialog(
    mut reader: MessageReader<OpenFileDialogEvent>,
    mut load_level_writer: MessageWriter<LoadLevelEvent>,
) {
    for message in reader.read() {
        if let Some(file_path) = FileDialog::new()
            .set_title("Select a .json level file")
            .add_filter("Level files", &["json"])
            .set_directory(message.directory.clone())
            .pick_file()
        {
            info!("Player selected level at path: {:?}", file_path);
            load_level_writer.write(LoadLevelEvent {
                path: file_path.to_string_lossy().to_string(),
            });
        }
    }
}
