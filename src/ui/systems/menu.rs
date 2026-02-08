use std::process::exit;

use bevy::{
    asset::Handle,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        hierarchy::ChildOf,
        query::{Changed, With},
        relationship::RelatedSpawnerCommands,
        system::{Commands, Query, ResMut},
    },
    state::state::NextState,
    text::{Font, TextColor, TextFont},
    ui::{
        AlignItems, BackgroundColor, Interaction, JustifyContent, Node, Val,
        widget::{Button, Text},
    },
};

use crate::{
    states::{GameState, MenuState},
    ui::components::MenuAction,
};

// Type denoting buttons that have been interacted with this frame (hover or press)
type ButtonsInteractedWith = (Changed<Interaction>, With<Button>);

pub fn menu_button_system(
    mut interaction_query: Query<(&Interaction, &MenuAction), ButtonsInteractedWith>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                MenuAction::Play => game_state.set(GameState::Playing),
                MenuAction::Quit => exit(0),
                MenuAction::BackToMainMenu => {
                    game_state.set(GameState::Menu);
                    menu_state.set(MenuState::Main)
                }
            }
        }
    }
}

pub fn spawn_button(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    font: &Handle<Font>,
    text: &str,
    action: MenuAction,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.35)),
            action,
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

pub fn despawn_with<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
