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
    time::{Time, Timer, TimerMode},
    ui::{Node, UiRect, Val, widget::Text},
};

use crate::{
    game::states::{GameState, MenuState},
    ui::{components::WinEntity, resources::WinTimer},
};

pub fn win_screen_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(WinTimer(Timer::from_seconds(3.0, TimerMode::Once)));
    commands.spawn((
        Camera2d,
        WinEntity,
        Text::new("win"),
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

pub fn win_timer(
    time: Res<Time>,
    mut timer: ResMut<WinTimer>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    timer.0.tick(time.delta());
    if timer.0.is_finished() {
        game_state.set(GameState::Menu);
        menu_state.set(MenuState::Main);
    }
}

pub fn cleanup_win(mut commands: Commands, query: Query<Entity, With<WinEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
