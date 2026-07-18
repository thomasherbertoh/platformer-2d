use bevy::{
    asset::AssetServer,
    camera::Camera2d,
    color::Color,
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    text::{TextColor, TextFont},
    time::{Timer, TimerMode},
    ui::{Node, UiRect, Val, widget::Text},
};

use crate::{
    game::states::{GameState, MenuState},
    ui::{components::SplashEntity, resources::GameTimer},
};

pub fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameTimer {
        timer: Timer::from_seconds(3.0, TimerMode::Once),
        target_state: GameState::SplashScreen,
        next_game_state: GameState::Menu,
        next_menu_state: MenuState::Main,
    });
    commands.spawn((
        Camera2d,
        SplashEntity,
        Text::new("2D Platformer B-)"),
        TextFont {
            font: asset_server.load("fonts/Roboto-Regular.ttf"),
            font_size: 64.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        Node {
            margin: UiRect::all(Val::Auto),
            ..Default::default()
        },
    ));
}

pub fn cleanup_splash(mut commands: Commands, query: Query<Entity, With<SplashEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
