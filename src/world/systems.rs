use std::fs;

use bevy::{
    color::Color,
    ecs::{
        entity::Entity,
        message::MessageReader,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    log::info,
    math::{Vec2, Vec3},
    sprite::Sprite,
    transform::components::{GlobalTransform, Transform},
};
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, GravityScale, LockedAxes, RigidBody, Sensor, Velocity,
};

use crate::{
    game::{
        events::LoadLevelEvent,
        resources::{Colours, Config, Dimensions},
    },
    player::components::{FootSensor, OnGround, Player},
    world::{
        components::{
            Block,
            BlockType::{End, Floor, PlayerSpawn},
            EndGate, Ground, World, WorldBoundary,
        },
        resources::{LevelData, LevelFile, WorldBounds},
    },
};

pub fn build_world(
    mut commands: Commands,
    mut level_data: ResMut<LevelData>,
    level_file: Res<LevelFile>,
    config: Res<Config>,
) {
    info!("Attempting to build the world...");
    merge_blocks(&mut level_data);

    for block in &level_data.blocks {
        match block.block_type {
            Floor => spawn_block(&mut commands, block.pos, block.size),
            PlayerSpawn => spawn_player(
                &mut commands,
                block.pos,
                &config.dimensions,
                &config.colours,
            ),
            End => spawn_level_end(
                &mut commands,
                block.pos,
                &config.dimensions,
                &config.colours,
            ),
        }
    }

    spawn_world_boundaries(&mut commands, &level_data.world_bounds);
    info!("Successfully built the world.");

    // save the processed level
    save_level_data(&level_data, Some(&level_file.path));
}

fn spawn_world_boundaries(commands: &mut Commands, bounds: &WorldBounds) {
    // bottom boundary - placed at min y, stretched between min x and max x
    commands.spawn((
        WorldBoundary,
        World,
        Collider::cuboid((bounds.max.x - bounds.min.x) / 2.0, 5.0),
        Transform::from_xyz(0.0, bounds.min.y, 0.0),
        GlobalTransform::default(),
    ));

    // top boundary - placed at max y, stretched between min x and max x
    commands.spawn((
        WorldBoundary,
        World,
        Collider::cuboid((bounds.max.x - bounds.min.x) / 2.0, 5.0),
        Transform::from_xyz(0.0, bounds.max.y, 0.0),
        GlobalTransform::default(),
    ));

    // left boundary - placed at min x, stretched between min y and max y
    commands.spawn((
        WorldBoundary,
        World,
        Collider::cuboid(5.0, (bounds.max.y - bounds.min.y) / 2.0),
        Transform::from_xyz(bounds.min.x, 0.0, 0.0),
        GlobalTransform::default(),
    ));

    // right boundary - placed at max x, stretched between min y and max y
    commands.spawn((
        WorldBoundary,
        World,
        Collider::cuboid(5.0, (bounds.max.y - bounds.min.y) / 2.0),
        Transform::from_xyz(bounds.max.x, 0.0, 0.0),
        GlobalTransform::default(),
    ));
}

fn merge_blocks(level_data: &mut ResMut<LevelData>) {
    let mut floor_blocks = Vec::new();
    let mut non_floor_blocks = Vec::new();

    level_data
        .blocks
        .iter()
        .for_each(|block| match block.block_type {
            Floor => floor_blocks.push(block.clone()),
            _ => non_floor_blocks.push(block.clone()),
        });

    let mut blocks = merge_blocks_horizontally(floor_blocks);
    blocks.extend(non_floor_blocks);
    level_data.blocks = blocks;
}

fn spawn_player(commands: &mut Commands, pos: Vec3, dimensions: &Dimensions, colours: &Colours) {
    let player_colour = &colours.player_colour;
    commands
        .spawn((
            Player,
            World,
            OnGround(false),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            GravityScale(1.0),
            Collider::cuboid(
                dimensions.player_width / 2.0,
                dimensions.player_height / 2.0,
            ),
            ActiveEvents::COLLISION_EVENTS,
            Sprite {
                color: Color::linear_rgb(player_colour.r, player_colour.g, player_colour.b),
                custom_size: Some(Vec2::new(dimensions.player_width, dimensions.player_height)),
                ..Default::default()
            },
            Transform::from_translation(pos),
            GlobalTransform::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                FootSensor,
                World,
                Sensor,
                Collider::cuboid(
                    dimensions.player_width * dimensions.width_foot_sensor_scale_factor,
                    dimensions.player_height * dimensions.height_foot_sensor_scale_factor,
                ),
                ActiveEvents::COLLISION_EVENTS,
                Transform::from_xyz(0.0, -((dimensions.player_height / 2.0) + 1.0), 0.0),
                GlobalTransform::default(),
            ));
        });
}

fn spawn_level_end(commands: &mut Commands, pos: Vec3, dimensions: &Dimensions, colours: &Colours) {
    let level_end_colour = &colours.level_end_colour;
    commands.spawn((
        EndGate,
        World,
        Collider::cuboid(
            (dimensions.player_width * dimensions.end_gate_scale_factor) / 2.0,
            (dimensions.player_height * dimensions.end_gate_scale_factor) / 2.0,
        ),
        ActiveEvents::COLLISION_EVENTS,
        Sprite {
            color: Color::linear_rgba(
                level_end_colour.r,
                level_end_colour.g,
                level_end_colour.b,
                level_end_colour.a,
            ),
            custom_size: Some(Vec2::new(
                dimensions.player_width * dimensions.end_gate_scale_factor,
                dimensions.player_height * dimensions.end_gate_scale_factor,
            )),
            ..Default::default()
        },
        Transform::from_translation(pos),
        GlobalTransform::default(),
    ));
}

fn spawn_block(commands: &mut Commands, pos: Vec3, block_size: Vec2) {
    commands.spawn((
        Ground,
        World,
        Collider::cuboid(block_size.x / 2.0, block_size.y / 2.0),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(block_size),
            ..Default::default()
        },
        Transform::from_translation(pos),
        GlobalTransform::default(),
    ));
}

fn load_level_data(level_file: &LevelFile) -> LevelData {
    let level = fs::read_to_string(&level_file.path)
        .unwrap_or_else(|_| panic!("Failed to read level file: `{}`", level_file.path));
    serde_json::from_str(&level)
        .unwrap_or_else(|_| panic!("Failed to parse level file: `{}`", level_file.path))
}

fn do_setup_level(commands: &mut Commands, level_file: &LevelFile) {
    info!("Attempting to fetch level `{}`", &level_file.path);
    commands.insert_resource(load_level_data(level_file));
    info!("Successfully fetched level `{}`", &level_file.path);
}

pub fn setup_level(mut commands: Commands, level_file: Res<LevelFile>) {
    do_setup_level(&mut commands, &level_file);
}

fn update_level_file(commands: &mut Commands, file_path: String) {
    let level_file = LevelFile { path: file_path };
    commands.insert_resource(level_file.clone());
    do_setup_level(commands, &level_file);
}

pub fn handle_load_level(mut commands: Commands, mut reader: MessageReader<LoadLevelEvent>) {
    for message in reader.read() {
        update_level_file(&mut commands, message.path.clone());
    }
}

fn save_level_data(level_data: &LevelData, filename: Option<&str>) {
    if let Ok(serialised_level_data) = serde_json::to_string(&level_data) {
        match fs::write(
            filename.unwrap_or("assets/levels/level.json"),
            serialised_level_data,
        ) {
            Ok(_) => {}
            Err(e) => println!("Error occurred while serialising level: {}", e),
        }
    }
}

fn can_merge_horizontally(a: &Block, b: &Block) -> bool {
    // Note: a block's x position is its centre => half of its size in the x dimension is to the left, half to the right
    a.pos.y == b.pos.y
        && a.size.y == b.size.y
        && (a.pos.x + (a.size.x / 2.0) == (b.pos.x - (b.size.x / 2.0)))
        && a.block_type == b.block_type
}

fn merge_horizontally(a: &Block, b: &Block) -> Block {
    let left = (a.pos.x - a.size.x / 2.0).min(b.pos.x - b.size.x / 2.0);
    let right = (a.pos.x + a.size.x / 2.0).max(b.pos.x + b.size.x / 2.0);
    let new_size = Vec2::new(right - left, a.size.y);
    let new_pos = Vec3::new((left + right) / 2.0, a.pos.y, a.pos.z);
    Block::new(new_pos, new_size, a.block_type.clone())
}

fn merge_blocks_horizontally(mut blocks: Vec<Block>) -> Vec<Block> {
    let mut merged = true;

    while merged {
        merged = false;
        'outer: for i in 0..blocks.len() {
            for j in (i + 1)..blocks.len() {
                if can_merge_horizontally(&blocks[i], &blocks[j]) {
                    let new_block = merge_horizontally(&blocks[i], &blocks[j]);
                    blocks.remove(j);
                    blocks[i] = new_block;
                    merged = true;
                    break 'outer;
                }
            }
        }
    }
    blocks
}

pub fn cleanup_world(mut commands: Commands, query: Query<Entity, With<World>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
