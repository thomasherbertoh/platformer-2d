use bevy::{ecs::component::Component, time::Timer};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct OnGround(pub bool);

#[derive(Component)]
pub struct FootSensor;

/**
 * Timer used to keep track of how much time has passed since the player walked off an edge
 * Solves cases where the player feels like they jumped before leaving the block, but actually jumped too late
 */
#[derive(Component)]
pub struct CoyoteTime {
    pub timer: Timer,
}

impl CoyoteTime {
    pub fn consume(&mut self) {
        self.timer.set_elapsed(self.timer.duration());
    }
}

/**
 * Timer used to keep track of recent jumps
 * Solves cases where the player feels like they jumped when they landed, but actually jumped too early
 */
#[derive(Component)]
pub struct JumpBuffer {
    pub timer: Timer,
}

impl JumpBuffer {
    pub fn consume(&mut self) {
        self.timer.set_elapsed(self.timer.duration());
    }
}
