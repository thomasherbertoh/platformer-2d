use bevy::ecs::component::Component;

#[derive(Component)]
pub enum MenuAction {
    Play,
    Quit,
    BackToMainMenu,
    LevelSelect,
}

#[derive(Component, Clone, Copy)]
pub struct GameOverMenuUI;

#[derive(Component, Clone, Copy)]
pub struct MainMenuUI;

#[derive(Component, Clone, Copy)]
pub struct PauseMenuUI;

#[derive(Component)]
pub struct SplashEntity;

#[derive(Component)]
pub struct WinEntity;
