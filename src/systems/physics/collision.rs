use bevy::{
    ecs::{
        query::{With, Without},
        system::Query,
    },
    transform::components::Transform,
};

use crate::{
    components::{
        physics::{Collider, Velocity},
        tags::{Ground, Player},
    },
    util::maths::{aabb_intersects, check_vertical_penetration},
};

pub fn player_ground_collision(
    mut player_query: Query<(&mut Transform, &Collider, &mut Velocity), With<Player>>,
    ground_query: Query<(&Transform, &Collider), (With<Ground>, Without<Player>)>,
) {
    if let Ok((mut player_transform, player_collider, mut velocity)) = player_query.single_mut() {
        let player_pos = player_transform.translation.truncate();

        for (ground_transform, ground_collider) in &ground_query {
            let ground_pos = ground_transform.translation.truncate();

            if aabb_intersects(
                player_pos,
                player_collider.size,
                ground_pos,
                ground_collider.size,
            ) && velocity.0.y < 0.0
                && check_vertical_penetration(
                    ground_collider.size.y,
                    player_collider.size.y,
                    player_pos.y,
                    ground_pos.y,
                )
            {
                player_transform.translation.y =
                    ground_pos.y + ground_collider.size.y / 2.0 + player_collider.size.y / 2.0;
                velocity.0.y = -velocity.0.y * 0.1;
            }
        }
    }
}
