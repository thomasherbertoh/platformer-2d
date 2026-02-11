use bevy::{
    asset::{AssetServer, Handle},
    camera::Camera2d,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    text::Font,
    ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val},
};

use crate::ui::{components::MenuAction, systems::button::spawn_button};

pub trait Menu {
    fn spawn_menu(commands: Commands, asset_server: Res<AssetServer>);
}

pub fn do_spawn_menu<T: Component>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_details: Vec<(&str, MenuAction)>,
    menu_type: T,
) {
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
            menu_type,
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
                    for (text, menu_action) in button_details {
                        spawn_button(menu, &font, text, menu_action);
                    }
                });
        });
}

pub fn despawn_with<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
