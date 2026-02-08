use bevy::ecs::component::Component;

#[derive(Component)]
pub enum MenuAction {
    Play,
    Quit,
    BackToMainMenu,
}

#[derive(Component)]
pub struct SplashEntity;

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct WinEntity;

#[derive(Component)]
pub struct GameOverMenuUI;
