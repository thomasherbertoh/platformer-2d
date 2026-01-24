use bevy::{ecs::resource::Resource, time::Timer};

#[derive(Resource)]
pub struct WinTimer(pub Timer);
