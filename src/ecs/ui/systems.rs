use bevy_egui::{egui, EguiContexts};

use bevy::{
    core::Name,
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

use crate::{
    ecs::{
        agent::*,
        components::{ConsumeTask, DurationActionMarker, Idle},
        trade::components::{BuyTask, Buying, TradeNegotiation},
    },
    AgentInteractionQueue, Walking,
};
use crate::{
    ecs::{
        components::{Interacting, WaitingInteraction},
        trade::components::Selling,
        ui::resources::SelectedAgent,
    },
    Consuming,
};

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

pub fn agent_ui_panel_system(
    mut contexts: EguiContexts,
    selected_agent: Res<SelectedAgent>,
    // We need queries for all the data we want to display
    agent_query: Query<(&Agent, &Name, &AgentInteractionQueue)>,
    // Queries for action/state components
    idle_query: Query<&Idle>,
    walking_query: Query<&Walking>,
    buying_query: Query<&Buying>,
    selling_query: Query<&Selling>,
    consuming_query: Query<&Consuming>,
    trade_query: Query<&TradeNegotiation>, // The active interaction component
    task_query: Query<(Option<&BuyTask>, Option<&ConsumeTask>)>,
    interaction_query: Query<(Option<&Interacting>, Option<&WaitingInteraction>)>,
) {
    // Check if an agent is selected. If not, we don't draw anything.
    let Some(selected_entity) = selected_agent.entity else {
        return;
    };

    // Attempt to get the main agent data. If this fails, the entity might have been despawned.
    let Ok((agent, name, interaction_queue)) = agent_query.get(selected_entity) else {
        return;
    };

    // --- This is where we define the UI panel ---
    egui::SidePanel::right("agent_info_panel")
        .default_width(250.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading(format!("Inspector: {}", name.as_str()));
            ui.separator();

            // --- Display Current Status ---
            ui.label("CURRENT STATUS:");
            let status_text = if idle_query.get(selected_entity).is_ok() {
                "State: Idle 😴".to_string()
            } else if let Ok(walking) = walking_query.get(selected_entity) {
                format!(
                    "State: Walking to [{:.1}, {:.1}] 🚶",
                    walking.destination.x, walking.destination.y
                )
            } else if buying_query.get(selected_entity).is_ok() {
                "State: Preparing to Buy 🛒".to_string()
            } else if selling_query.get(selected_entity).is_ok() {
                "State: Selling 💰".to_string()
            } else if consuming_query.get(selected_entity).is_ok() {
                "State: Consuming 🍔".to_string()
            } else if let Ok(trade) = trade_query.get(selected_entity) {
                format!("State: Trading ({:?}) 🤝", trade)
            } else {
                "State: Unknown".to_string()
            };
            ui.label(status_text);
            ui.separator();

            // --- Display Task ---
            ui.label("CURRENT TASK:");
            if let Ok((buy, consume)) = task_query.get(selected_entity) {
                if let Some(_) = buy {
                    ui.label("Buy Task");
                } else if let Some(_) = consume {
                    ui.label("Consume Task");
                } else {
                    ui.label("No task");
                }
            }

            // --- Display Agent's Role & Needs ---
            ui.label("DETAILS:");
            ui.label(format!("Hunger: {:.1}/100", agent.needs.hunger));
            ui.label(format!("Thirst: {:.1}/100", agent.needs.thirst));
            ui.separator();

            // --- Display Inventory ---
            ui.label("INVENTORY:");
            let items_list = &agent.inventory.list();
            for (item, quantity) in items_list {
                ui.label(format!("- {} (x{})", format!("{:?}", item), quantity));
            }
            if items_list.is_empty() {
                ui.label("- Empty");
            }
            ui.separator();

            // --- Display Next Action in Plan ---
            ui.label("ACTION PLAN:");
            if let Some(next_action) = agent.get_action() {
                // Assuming you still have this for planning
                ui.label(format!("- {:?}", next_action));
            } else {
                ui.label("- No current plan");
            }
            ui.separator();

            // --- Display Interaction Queue ---
            ui.label("INTERACTION QUEUE:");
            ui.label(format!("Current size: {:?}", interaction_queue.len()));
            if let Ok((interacting, waiting_interaction)) = interaction_query.get(selected_entity) {
                if let Some(_) = interacting {
                    ui.label("Interacting");
                } else if let Some(w) = waiting_interaction {
                    ui.label(format!("Waiting Interaction {}", w.get_resting_duration()));
                } else {
                    ui.label("No interaction");
                }
            }
        });
}
