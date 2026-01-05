mod components;
mod game;
mod plugins;
mod resources;
mod states;
mod systems;
mod util;

use crate::game::GamePlugin;
use bevy::{DefaultPlugins, app::App};

fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin)).run();
}
