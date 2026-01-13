use bevy::{
    asset::AssetServer,
    camera::Camera2d,
    color::Color,
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    state::state::NextState,
    text::{TextColor, TextFont},
    time::{Time, Timer, TimerMode::Once},
    ui::{Node, UiRect, Val, widget::Text},
};

use crate::{components::tags::SplashEntity, resources::splash::SplashTimer, states::GameState};

pub fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, Once)));
    commands.spawn((Camera2d, SplashEntity));
    commands.spawn((
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
        SplashEntity,
    ));
}

pub fn splash_timer(
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());
    if timer.0.is_finished() {
        next_state.set(GameState::MainMenu);
    }
}

pub fn cleanup_splash(mut commands: Commands, query: Query<Entity, With<SplashEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
