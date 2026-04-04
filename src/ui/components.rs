use bevy::ecs::component::Component;

#[derive(Component)]
pub enum MenuAction {
    Play,
    Quit,
    BackToMainMenu,
    LevelSelect,
}

#[derive(Component)]
pub struct GameOverMenuUI;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct PauseMenuUI;

#[derive(Component)]
pub struct SplashEntity;

#[derive(Component)]
pub struct WinEntity;
