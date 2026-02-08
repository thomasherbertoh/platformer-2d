use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
    math::{Vec2, Vec3},
    state::{condition::in_state, state::OnEnter},
};

use crate::{
    camera::systems::{
        center_camera_on_world, spawn_world_camera, update_camera_projection_on_resize,
    },
    states::GameState,
    world::{
        components::{Block, BlockType},
        resources::{LevelData, WorldBounds},
        systems::{build_world, save_level},
    },
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        let mut blocks = Vec::new();
        for i in 0..10 {
            blocks.push(Block::new(
                Vec3::new(i as f32 * 10.0, 0.0, 0.0),
                Vec2::new(10.0, 10.0),
                BlockType::Floor,
            ));
        }
        for i in 0..10 {
            blocks.push(Block::new(
                Vec3::new(100.0 + i as f32 * 10.0, 50.0, 0.0),
                Vec2::new(10.0, 10.0),
                BlockType::Floor,
            ));
        }
        for i in 0..10 {
            blocks.push(Block::new(
                Vec3::new(-100.0 + i as f32 * 10.0, 50.0, 0.0),
                Vec2::new(10.0, 10.0),
                BlockType::Floor,
            ));
        }
        blocks.push(Block::new(
            Vec3::new(0.0, 10.0, 0.0),
            Vec2::new(10.0, 10.0),
            BlockType::PlayerSpawn,
        ));
        blocks.push(Block::new(
            Vec3::new(190.0, 67.5, 0.0),
            Vec2::new(10.0, 10.0),
            BlockType::End,
        ));

        app.insert_resource(LevelData {
            blocks,
            world_bounds: WorldBounds {
                min: Vec2::new(-500.0, -300.0),
                max: Vec2::new(500.0, 300.0),
            },
        })
        .add_systems(
            OnEnter(GameState::Playing),
            (save_level, build_world, spawn_world_camera),
        )
        .add_systems(
            Update,
            ((center_camera_on_world, update_camera_projection_on_resize)
                .run_if(in_state(GameState::Playing)),),
        );
    }
}
