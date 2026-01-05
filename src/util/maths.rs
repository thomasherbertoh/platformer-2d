use bevy::math::Vec2;

pub fn aabb_intersects(pos_a: Vec2, size_a: Vec2, pos_b: Vec2, size_b: Vec2) -> bool {
    let a_min = pos_a - size_a / 2.0;
    let a_max = pos_a + size_a / 2.0;

    let b_min = pos_b - size_b / 2.0;
    let b_max = pos_b + size_b / 2.0;

    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}

pub fn check_vertical_penetration(
    ground_collider_size_y: f32,
    player_collider_size_y: f32,
    player_pos_y: f32,
    ground_pos_y: f32,
) -> bool {
    (ground_collider_size_y / 2.0 * player_collider_size_y / 2.0)
        - (player_pos_y - ground_pos_y).abs()
        > 0.0
}
