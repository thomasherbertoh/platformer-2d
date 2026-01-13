use bevy::{ecs::resource::Resource, time::Timer};

#[derive(Resource)]
pub struct SplashTimer(pub Timer);
