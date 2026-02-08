mod camera;
mod game;
mod player;
mod states;
mod ui;
mod world;

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
