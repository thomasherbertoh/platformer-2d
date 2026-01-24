use bevy::{
    ecs::{
        message::MessageReader,
        query::With,
        system::{Query, Res, ResMut},
    },
    state::state::NextState,
};
use bevy_rapier2d::prelude::{CollidingEntities, CollisionEvent, Velocity};

use crate::{
    components::tags::{EndGate, FootSensor, OnGround, Player},
    resources::ground_contacts::GroundContacts,
    states::GameState,
};

pub fn foot_sensor_collision_system(
    mut collision_events: MessageReader<CollisionEvent>,
    mut contacts: ResMut<GroundContacts>,
    sensors: Query<(), With<FootSensor>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                if sensors.get(*e1).is_ok() {
                    contacts.sensors.insert(*e1);
                }
                if sensors.get(*e2).is_ok() {
                    contacts.sensors.insert(*e2);
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                contacts.sensors.remove(e1);
                contacts.sensors.remove(e2);
            }
        }
    }
}

pub fn update_grounded_state(
    mut players: Query<&mut OnGround, With<Player>>,
    contacts: Res<GroundContacts>,
) {
    let grounded = !contacts.sensors.is_empty();
    for mut on_ground in &mut players {
        on_ground.0 = grounded;
    }
}

pub fn clamp_velocity_when_grounded(mut query: Query<(&mut Velocity, &OnGround), With<Player>>) {
    for (mut velocity, on_ground) in &mut query {
        if on_ground.0 && velocity.linvel.y < 0.0 {
            velocity.linvel.y = 0.0;
        }
    }
}

pub fn ground_detection_system(
    mut query: Query<&mut OnGround, With<Player>>,
    sensor_query: Query<&CollidingEntities, With<FootSensor>>,
) {
    let mut grounded = false;

    if let Ok(colliding) = sensor_query.single() {
        grounded = !colliding.is_empty();
    }

    if let Ok(mut on_ground) = query.single_mut() {
        on_ground.0 = grounded;
    }
}

pub fn end_gate_collision_system(
    mut collision_events: MessageReader<CollisionEvent>,
    end_gate: Query<(), With<EndGate>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            // if the end_gate is being collided with then we win
            // presumably this will break if other stuff collides with the end gate
            if end_gate.get(*e1).is_ok() || end_gate.get(*e2).is_ok() {
                next_state.set(GameState::Win);
            }
        };
    }
}
