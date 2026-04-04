use std::path::PathBuf;

use bevy::ecs::message::Message;

#[derive(Message)]
pub struct LoadLevelEvent {
    pub path: String,
}

#[derive(Message)]
pub struct OpenFileDialogEvent {
    pub directory: PathBuf,
}
