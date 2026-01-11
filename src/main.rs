mod components;
mod game;
mod plugins;
mod resources;
mod states;
mod systems;

use crate::game::GamePlugin;
use bevy::{DefaultPlugins, app::App};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            // RapierDebugRenderPlugin::default(),  // enable for debug graphics
            GamePlugin,
        ))
        .run();
}
