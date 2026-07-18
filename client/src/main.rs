mod camera;
mod game;
mod player;
mod ui;
mod world;

use std::env;

use crate::{
    game::{events::FetchLevelEvent, plugin::GamePlugin, resources::LevelChannel},
    world::resources::LevelFile,
};
use bevy::{DefaultPlugins, app::App};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use crossbeam_channel::unbounded;

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

    let (tx, rx) = unbounded();

    App::new()
        .insert_resource(LevelFile { path: level_path })
        .insert_resource(LevelChannel {
            sender: tx,
            receiver: rx,
        })
        .add_message::<FetchLevelEvent>()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            // RapierDebugRenderPlugin::default(),  // enable for debug graphics
            GamePlugin,
        ))
        .run();
}
