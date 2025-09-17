use bevy::{
    ecs::{
        entity::Entity,
        query::With,
        system::{Query, Res, ResMut},
    },
    input::{mouse::MouseButton, ButtonInput},
    math::{primitives::InfinitePlane3d, Dir3, Vec3},
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

use crate::ecs::ui::resources::SelectedAgent;

pub fn agent_selection_system(
    mut selected_agent: ResMut<SelectedAgent>,
    // 1. Query for the primary window specifically
    window_query: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    agent_query: Query<(Entity, &Transform)>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = window_query.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = cameras.get_single() else {
        return;
    };

    // 2. The new, ray-based method for cursor-to-world conversion
    let world_pos = window
        .cursor_position()
        .and_then(|cursor_position| {
            camera
                .viewport_to_world(camera_transform, cursor_position)
                .ok()
        })
        .and_then(|ray| {
            // Find where the ray intersects the XY plane (z=0)
            ray.intersect_plane(Vec3::ZERO, InfinitePlane3d { normal: Dir3::Z })
                .map(|distance| ray.get_point(distance))
        });

    if let Some(world_pos) = world_pos {
        let mut clicked_on_agent = false;
        for (agent_entity, agent_transform) in &agent_query {
            // The rest of the logic is the same, just using the new world_pos
            let distance = world_pos
                .truncate()
                .distance(agent_transform.translation.truncate());
            if distance < 32.0 {
                println!("Selected agent {:?}", agent_entity);
                selected_agent.entity = Some(agent_entity);
                clicked_on_agent = true;
                break;
            }
        }

        if !clicked_on_agent {
            selected_agent.entity = None;
        }
    }
}
