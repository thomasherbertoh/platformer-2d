use std::collections::HashSet;

use bevy::ecs::{entity::Entity, resource::Resource};

#[derive(Resource, Default)]
pub struct GroundContacts {
    pub sensors: HashSet<Entity>,
}
