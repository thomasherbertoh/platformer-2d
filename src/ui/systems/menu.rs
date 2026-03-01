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

use crate::{
    game::resources::{Colours, Config},
    ui::{components::MenuAction, systems::button::spawn_button},
};

pub trait Menu {
    fn spawn_menu(commands: Commands, asset_server: Res<AssetServer>, config: Res<Config>);
}

pub fn do_spawn_menu<T: Component>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_details: Vec<(&str, MenuAction)>,
    menu_type: T,
    colours: &Colours,
) {
    let font: Handle<Font> = asset_server.load("fonts/Roboto-Regular.ttf");

    let menu_outer_background_colour = &colours.menu_outer_background_colour;
    let menu_inner_background_colour = &colours.menu_inner_background_colour;

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
            BackgroundColor(Color::srgb(
                menu_outer_background_colour.r,
                menu_outer_background_colour.g,
                menu_outer_background_colour.b,
            )),
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
                    BackgroundColor(Color::srgb(
                        menu_inner_background_colour.r,
                        menu_inner_background_colour.g,
                        menu_inner_background_colour.b,
                    )),
                ))
                .with_children(|menu| {
                    for (text, menu_action) in button_details {
                        spawn_button(menu, &font, text, menu_action, colours);
                    }
                });
        });
}

pub fn despawn_with<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
