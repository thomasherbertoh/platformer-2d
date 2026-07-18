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
    text::{Font, Justify, TextColor, TextFont, TextLayout},
    ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val, widget::Text},
};

use crate::{
    game::resources::{Colours, Config},
    ui::{components::MenuAction, resources::HeadingText, systems::button::spawn_button},
};

pub trait Menu {
    fn spawn_menu(commands: Commands, asset_server: Res<AssetServer>, config: Res<Config>);
}

pub fn do_spawn_menu<T: Component + Copy>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_details: Vec<(&str, MenuAction)>,
    menu_type: T,
    colours: &Colours,
    heading: String,
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
                justify_content: JustifyContent::SpaceEvenly,
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
                        height: Val::Px(100.0 * (button_details.len() + 1) as f32),
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
                    menu.spawn((
                        Text::new(heading),
                        TextLayout::new_with_justify(Justify::Center),
                        TextFont {
                            font: asset_server.load("fonts/Roboto-Regular.ttf"),
                            font_size: 32.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                        HeadingText,
                        menu_type,
                    ));
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
