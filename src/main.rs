mod camera;
mod game;
mod player;
mod ui;
mod world;

use bevy::{DefaultPlugins, app::App};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

use crate::game::plugin::GamePlugin;

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
