use bevy::prelude::Message;
use std::path::PathBuf;

#[derive(Message)]
pub struct LoadLevelEvent {
    pub path: String,
}

#[derive(Message)]
pub struct OpenFileDialogEvent {
    pub directory: PathBuf,
}

#[derive(Message)]
pub struct FetchLevelEvent;
