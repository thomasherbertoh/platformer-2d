use bevy::{
    camera::{Camera, Camera2d, OrthographicProjection, Projection, ScalingMode},
    ecs::{
        message::MessageReader,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    transform::components::{GlobalTransform, Transform},
    window::{Window, WindowResized},
};

use crate::{
    components::tags::{Player, World},
    resources::world::WorldBounds,
};

pub fn spawn_world_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        World,
        Camera::default(),
        Projection::Orthographic(OrthographicProjection {
            // I think this is what causes the world resizing bug when you move the game from one size window to another - the size is calculated differently later
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 600.0,
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::default(),
        GlobalTransform::default(),
    ));
}

pub fn center_camera_on_world(
    bounds: Res<WorldBounds>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(mut transform) = camera_query.single_mut() {
        let center = (bounds.min + bounds.max) / 2.0;
        transform.translation.x = center.x;
        transform.translation.y = center.y;
    }
}

pub fn update_camera_projection_on_resize(
    resize_events: MessageReader<WindowResized>,
    windows: Query<&Window>,
    bounds: Res<WorldBounds>,
    mut query: Query<&mut Projection, With<Camera2d>>,
) {
    if resize_events.is_empty() {
        return;
    }

    if let Ok(window) = windows.single()
        && let Ok(mut projection) = query.single_mut()
    {
        let world_height = bounds.max.y - bounds.min.y;

        if let Projection::Orthographic(ref mut ortho) = *projection {
            ortho.scale = world_height / window.resolution.height();
        }
    }
}

pub fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.single()
        && let Ok(mut camera_transform) = camera_query.single_mut()
    {
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}
