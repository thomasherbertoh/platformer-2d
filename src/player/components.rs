use bevy::{ecs::component::Component, time::Timer};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct OnGround(pub bool);

#[derive(Component)]
pub struct FootSensor;

#[derive(Component)]
pub struct CoyoteTime {
    pub timer: Timer,
}

impl CoyoteTime {
    pub fn consume(&mut self) {
        self.timer.set_elapsed(self.timer.duration());
    }
}
