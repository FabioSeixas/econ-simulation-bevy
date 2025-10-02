use bevy_egui::{egui, EguiContexts};

use bevy::{
    color::{palettes::css::YELLOW, Color},
    core::Name,
    ecs::{
        entity::Entity,
        observer::Trigger,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{mouse::MouseButton, ButtonInput},
    math::{primitives::InfinitePlane3d, Dir3, Vec3},
    render::camera::Camera,
    sprite::Sprite,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

use crate::ecs::{
    buy::{actions::components::Buying, tasks::components::BuyTask},
    components::{Interacting, WaitingInteraction},
    consume::{actions::components::Consuming, tasks::components::ConsumeTask},
    sell::actions::components::Selling,
    talk::{interaction::components::KnowledgeSharingInteraction, task::components::TalkTask},
    trade::components::TradeNegotiation,
    ui::{events::ChangeSelectedEntity, resources::SelectedAgent},
};
use crate::{
    ecs::{
        agent::*,
        components::{DurationAction, Idle},
        logs::AgentLogs,
    },
    AgentInteractionQueue, Walking,
};

pub fn change_selected_entity(
    trigger: Trigger<ChangeSelectedEntity>,
    mut selected_agent: ResMut<SelectedAgent>,
    mut agent_query: Query<&mut Sprite>,
) {
    let previous_selected = selected_agent.entity.clone();

    if let Ok(mut sprite) = agent_query.get_mut(trigger.target) {
        let original_color = sprite.color;
        sprite.color = Color::srgb(YELLOW.red, YELLOW.green, YELLOW.blue);

        selected_agent.entity = Some((trigger.target, original_color));
    }

    if let Some((entity, original_color)) = previous_selected {
        if let Ok(mut sprite) = agent_query.get_mut(entity) {
            sprite.color = original_color;
        }
    }
}

pub fn agent_selection_system(
    mut contexts: EguiContexts,
    mut selected_agent: ResMut<SelectedAgent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut agent_query: Query<(Entity, &Name, &Transform, &mut Sprite)>,
) {
    if contexts.ctx_mut().wants_pointer_input() {
        return;
    }

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

        if let Some((entity, original_color)) = selected_agent.entity {
            if let Ok((_, _, _, mut sprite)) = agent_query.get_mut(entity) {
                sprite.color = original_color;
            }
        }

        for (agent_entity, name, agent_transform, mut sprite) in &mut agent_query {
            // The rest of the logic is the same, just using the new world_pos
            let distance = world_pos
                .truncate()
                .distance(agent_transform.translation.truncate());
            if distance < 32.0 {
                println!("Selected agent {:?} - {:?}", agent_entity, name);
                println!("Agent sprite color {:?}", sprite.color);

                let original_color = sprite.color;
                sprite.color = Color::srgb(YELLOW.red, YELLOW.green, YELLOW.blue);

                selected_agent.entity = Some((agent_entity, original_color));

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
    mut commands: Commands,
    selected_agent: Res<SelectedAgent>,
    agent_query: Query<(&Agent, &Name, &AgentInteractionQueue, &AgentLogs)>,
    action_query: Query<(
        Option<&Idle>,
        Option<&Consuming>,
        Option<&Selling>,
        Option<&Buying>,
        Option<&Walking>,
    )>,
    task_query: Query<(Option<&BuyTask>, Option<&ConsumeTask>, Option<&TalkTask>)>,
    interaction_query: Query<(Option<&Interacting>, Option<&WaitingInteraction>)>,
    interaction_data_query: Query<(
        Option<&TradeNegotiation>,
        Option<&KnowledgeSharingInteraction>,
    )>,
) {
    // Check if an agent is selected. If not, we don't draw anything.
    let Some((selected_entity, _)) = selected_agent.entity else {
        return;
    };

    // Attempt to get the main agent data. If this fails, the entity might have been despawned.
    let Ok((agent, name, interaction_queue, agent_memory)) = agent_query.get(selected_entity)
    else {
        return;
    };

    egui::SidePanel::right("agent_info_panel")
        .default_width(250.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading(format!("Inspector: {}", name.as_str()));
            ui.separator();

            ui.label("CURRENT MARKERS:");
            if let Ok((idle, consuming, selling, buying, walking)) =
                action_query.get(selected_entity)
            {
                if let Some(_) = idle {
                    ui.label("State: Idle 😴".to_string());
                }

                if let Some(v) = consuming {
                    ui.label(format!(
                        "State: Consuming 🍔 - {:.1}",
                        v.get_resting_duration()
                    ));
                }

                if let Some(v) = selling {
                    ui.label(format!(
                        "State: Selling 💰 - {:.1}",
                        v.get_resting_duration()
                    ));
                }

                if let Some(_) = buying {
                    ui.label("State: Buying 🛒".to_string());
                }

                if let Some(w) = walking {
                    ui.label(format!(
                        "State: Walking to [{:.1}, {:.1}] 🚶",
                        w.destination.x, w.destination.y
                    ));
                }
            }
            ui.separator();

            ui.label("CURRENT TASK:");
            if let Ok((buy, consume, knowledge)) = task_query.get(selected_entity) {
                if let Some(_) = buy {
                    ui.label("Buy Task");
                }

                if let Some(_) = consume {
                    ui.label("Consume Task");
                }

                if let Some(v) = knowledge {
                    ui.label("Obtain Knowledge Task");
                    ui.label(format!("Tried: {:?}", v.tried));
                    if let Some((id, _partner, name)) = &v.current_interaction {
                        ui.label(format!("Current interaction {} with {}", id, name));
                    }
                }
            }
            ui.separator();

            ui.label("CURRENT Interaction:");
            if let Ok((interacting, waiting_interaction)) = interaction_query.get(selected_entity) {
                if let Some(v) = interacting {
                    ui.label(format!(
                        "Interacting {} {:.1}",
                        v.id,
                        v.get_resting_duration()
                    ));
                }

                if let Some(w) = waiting_interaction {
                    ui.label(format!(
                        "Waiting Interaction {} {:.1}",
                        w.id,
                        w.get_resting_duration()
                    ));
                    if ui.button("Select partner").clicked() {
                        if w.target == selected_entity {
                            commands.trigger(ChangeSelectedEntity { target: w.source });
                        } else {
                            commands.trigger(ChangeSelectedEntity { target: w.target });
                        }
                    };
                }
            }
            ui.separator();

            ui.label("CURRENT Interaction Data:");
            if let Ok((trade, knowledge_interaction)) = interaction_data_query.get(selected_entity)
            {
                if let Some(_) = trade {
                    ui.label("TradeNegotiation");
                }

                if let Some(v) = knowledge_interaction {
                    ui.label(format!(
                        "KnowledgeSharingInteraction - source: {} - target: {}",
                        v.source_name, v.target_name
                    ));
                    if ui.button("Select partner").clicked() {
                        if v.target == selected_entity {
                            commands.trigger(ChangeSelectedEntity { target: v.source });
                        } else {
                            commands.trigger(ChangeSelectedEntity { target: v.target });
                        }
                    };
                }
            }
            ui.separator();

            // --- Display Agent's Role & Needs ---
            ui.label("DETAILS:");
            ui.label(format!("Hunger: {:.1}/1000", agent.needs.hunger));
            ui.label(format!("Thirst: {:.1}/1000", agent.needs.thirst));
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

            // --- Display Interaction Queue ---
            ui.label("INTERACTION QUEUE:");
            ui.label(format!("Current size: {:?}", interaction_queue.len()));
            for interaction_item in interaction_queue.list() {
                ui.label(format!("ID: {:?}", interaction_item.id));
            }
            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2]) // optional: prevent shrinking when content is small
                .show(ui, |ui| {
                    ui.label("LOGS:");
                    for entry in agent_memory.list().iter().rev() {
                        // ui.label(format!("{}: {}", &entry.time.as_secs(), &entry.description));
                        ui.label(format!("{}: {}", &entry.frame, &entry.description));
                    }
                });
        });
}
