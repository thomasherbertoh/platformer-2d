use bevy::{
    asset::{AssetServer, Handle},
    camera::Camera2d,
    color::Color,
    ecs::system::{Commands, Res},
    text::Font,
    ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val},
};

use crate::ui::{
    components::{GameOverMenuUI, MenuAction},
    systems::menu::spawn_button,
};

pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/Roboto-Regular.ttf");

    commands
        .spawn((
            Camera2d,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
            GameOverMenuUI,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(200.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                ))
                .with_children(|menu| {
                    spawn_button(menu, &font, "Retry", MenuAction::Play);
                    spawn_button(menu, &font, "Main Menu", MenuAction::BackToMainMenu);
                });
        });
}
