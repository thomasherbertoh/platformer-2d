use std::process::exit;

use bevy::{
    asset::{AssetServer, Handle},
    camera::Camera2d,
    color::Color,
    ecs::{
        entity::Entity,
        hierarchy::{ChildOf, Children},
        query::{Changed, With},
        relationship::RelatedSpawnerCommands,
        system::{Commands, Query, Res, ResMut},
    },
    state::state::NextState,
    text::{Font, TextColor, TextFont},
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, Node, Val,
        widget::{Button, Text},
    },
};

use crate::{components::tags::MenuUI, states::GameState};

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Roboto-Regular.ttf");

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
            MenuUI,
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
                    spawn_button(menu, &font, "Play");
                    spawn_button(menu, &font, "Quit");
                });
        });
}

fn spawn_button(menu: &mut RelatedSpawnerCommands<'_, ChildOf>, font: &Handle<Font>, text: &str) {
    menu.spawn((
        Button,
        Node {
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.25, 0.25, 0.35)),
    ))
    .with_children(|button| {
        button.spawn((
            Text::new(text),
            TextFont {
                font: font.clone(),
                font_size: 28.0,
                ..Default::default()
            },
            TextColor(Color::WHITE),
        ));
    });
}

// Type denoting buttons that have been interacted with this frame (hover or press)
type ButtonsInteractedWith = (Changed<Interaction>, With<Button>);

pub fn menu_button_system(
    mut interaction_query: Query<(&Interaction, &Children), ButtonsInteractedWith>,
    text_query: Query<&Text>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, children) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            let text = &text_query.get(children[0]).unwrap().0;
            match text.as_str() {
                "Play" => next_state.set(GameState::Playing),
                "Quit" => exit(0),
                _ => {}
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
