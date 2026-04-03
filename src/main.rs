mod camera;
mod game;
mod player;
mod ui;
mod world;

use std::env;

use bevy::{DefaultPlugins, app::App};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

use crate::{game::plugin::GamePlugin, world::resources::LevelFile};

fn main() {
    let args: Vec<String> = env::args().collect();

    let level_path = format!(
        "assets/levels/{}.json",
        if args.len() > 1 {
            args[1].clone()
        } else {
            "level".to_string()
        }
    );

    App::new()
        .insert_resource(LevelFile { path: level_path })
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            // RapierDebugRenderPlugin::default(),  // enable for debug graphics
            GamePlugin,
        ))
        .run();
}
