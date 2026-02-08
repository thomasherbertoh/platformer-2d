use bevy::ecs::component::Component;

#[derive(Component)]
pub struct SplashEntity;

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct OnGround(pub bool);

#[derive(Component)]
pub struct FootSensor;

#[derive(Component)]
pub struct WinEntity;

#[derive(Component)]
pub struct GameOverMenuUI;
