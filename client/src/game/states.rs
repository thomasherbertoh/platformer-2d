use bevy::state::state::{StateSet, States, SubStates};

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    SplashScreen,
    Menu,
    Playing,
    Win,
}

#[derive(SubStates, Debug, Clone, Eq, PartialEq, Hash, Default)]
#[source(GameState = GameState::Menu)]
pub enum MenuState {
    #[default]
    Main,
    GameOver,
    Paused,
}
