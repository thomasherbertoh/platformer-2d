use bevy::ecs::component::Component;

#[derive(Component)]
pub struct SplashEntity;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct OnGround(pub bool);

#[derive(Component)]
pub struct FootSensor;
