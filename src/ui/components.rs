use bevy::ecs::component::Component;

#[derive(Component)]
pub enum MenuAction {
    Play,
    Quit,
    BackToMainMenu,
}

#[derive(Component)]
pub struct GameOverMenuUI;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct SplashEntity;

#[derive(Component)]
pub struct WinEntity;
