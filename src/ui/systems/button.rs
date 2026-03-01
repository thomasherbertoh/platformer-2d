use std::process::exit;

use bevy::{
    asset::Handle,
    color::Color,
    ecs::{
        hierarchy::ChildOf,
        query::{Changed, With},
        relationship::RelatedSpawnerCommands,
        system::{Query, ResMut},
    },
    state::state::NextState,
    text::{Font, TextColor, TextFont},
    ui::{
        AlignItems, BackgroundColor, Interaction, JustifyContent, Node, Val,
        widget::{Button, Text},
    },
};

use crate::{
    game::{
        resources::Colours,
        states::{GameState, MenuState},
    },
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
                    menu_state.set(MenuState::Main);
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
    colours: &Colours,
) {
    let button_background_colour = &colours.button_background_colour;

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
            BackgroundColor(Color::srgb(
                button_background_colour.r,
                button_background_colour.g,
                button_background_colour.b,
            )),
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
