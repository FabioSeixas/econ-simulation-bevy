mod core;
mod ecs;

use bevy::log::*;
use bevy::prelude::*;

use crate::core::item::ItemEnum;
use crate::ecs::agent::*;
use crate::ecs::components::*;
use crate::ecs::consume::tasks::components::ConsumeTask;
use crate::ecs::interaction::*;
use crate::ecs::knowledge::KnowledgePlugin;
use crate::ecs::knowledge::SharedKnowledge;
use crate::ecs::logs::*;
use crate::ecs::roles::none::NoneRole;
use crate::ecs::roles::seller::SellerRole;
use crate::ecs::roles::{none::*, seller::*};
use crate::ecs::talk::ask::plugin::AskPlugin;
use crate::ecs::trade::components::*;
use crate::ecs::trade::plugin::TradePlugin;
use crate::ecs::trade::tasks::buy::components::BuyTask;
use crate::ecs::ui::plugin::UiPlugin;
use crate::ecs::utils::get_random_vec3;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG, // Set minimum level to show debug logs
            ..default()
        }))
        .add_event::<AddLogEntry>()
        .add_plugins(TradePlugin)
        .add_plugins(KnowledgePlugin)
        .add_plugins(AskPlugin)
        .add_plugins(UiPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                check_agent_interaction_queue_system,
                check_idle_agents_needs,
                handle_idle_sellers,
                handle_idle_none_role,
                handle_selling_action,
                handle_walking_action,
                obtain_knowledge_system,
                start_interaction_system,
            )
                .chain(),
        )
        .add_systems(Update, (update_agents, add_logs_system))
        .run();
}

pub fn add_logs_system(
    mut agent_query: Query<&mut AgentLogs, With<AgentLogs>>,
    mut add_logs_reader: EventReader<AddLogEntry>,
) {
    for event in add_logs_reader.read() {
        if let Ok(mut agent_logs) = agent_query.get_mut(event.target) {
            agent_logs.add(&event.description);
        }
    }
}

pub fn obtain_knowledge_system(
    mut source_agent_query: Query<
        (Entity, &Transform, &mut ObtainKnowledgeTask),
        (
            With<ObtainKnowledgeTask>,
            Without<Interacting>,
            Without<WaitingInteraction>,
        ),
    >,
    mut target_agent_query: Query<(Entity, &Transform, &mut AgentInteractionQueue), With<Agent>>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    for (source_entity, source_transform, mut obtain_knowledge_task) in &mut source_agent_query {
        if obtain_knowledge_task.current_interaction.is_some() {
            continue;
        }

        let mut best: Option<(Entity, f32)> = None;
        for (entity, target_transform, _) in &target_agent_query {
            if entity.eq(&source_entity) {
                continue;
            } 
            if obtain_knowledge_task.tried.contains(&entity) {
                continue;
            }

            let d2 = source_transform
                .translation
                .distance_squared(target_transform.translation);

            // TODO: set a maximum acceptable distance
            match best {
                None => best = Some((entity, d2)),
                Some((_, best_d2)) => {
                    if d2 < best_d2 {
                        best = Some((entity, d2))
                    }
                }
            }
        }

        if let Some((closest_entity, _)) = best {
            if let Ok((_, _, mut agent_interation_queue)) =
                target_agent_query.get_mut(closest_entity)
            {
                add_log_writer.send(AddLogEntry::new(
                    source_entity,
                    format!("Sent Ask Interaction request for {}", closest_entity).as_str(),
                ));

                obtain_knowledge_task.current_interaction = Some((
                    agent_interation_queue.add(AgentInteractionKind::Ask(
                        KnowledgeSharingInteraction {
                            seller_of: obtain_knowledge_task.content.seller_of,
                            partner: source_entity,
                        },
                    )),
                    closest_entity,
                ));

                commands
                    .entity(source_entity)
                    .insert(WaitingInteraction::new());
            }
        } else {
            // TODO
            info!("no agents found");
        }
    }
}

pub fn start_interaction_system(
    mut source_agent_query: Query<
        (Entity, &Transform, &mut ObtainKnowledgeTask),
        (
            With<ObtainKnowledgeTask>,
            With<WaitingInteraction>,
            Without<Walking>,
            Without<Interacting>,
        ),
    >,
    mut target_agent_query: Query<(Entity, &Transform, &mut AgentInteractionQueue), With<Agent>>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    for (source_entity, source_transform, obtain_knowledge_task) in &mut source_agent_query {
        if let Some((_, target_entity)) = obtain_knowledge_task.current_interaction {
            if let Ok((_, target_transform, _)) = target_agent_query.get_mut(target_entity) {
                if source_transform
                    .translation
                    .distance(target_transform.translation)
                    > 50.
                {
                    add_log_writer.send(AddLogEntry::new(
                        source_entity,
                        format!("Walking to for {}", target_entity).as_str(),
                    ));
                    let mut walking = Walking::new(
                        (target_transform.translation - source_transform.translation).normalize(),
                    );
                    walking.set_idle_at_completion(false);
                    commands.entity(source_entity).insert(walking);
                }
            }
        }
    }
}

fn update_agents(mut query: Query<&mut Agent, With<Agent>>) {
    for mut agent in &mut query {
        agent.frame_update();
    }
}

fn check_agent_interaction_queue_system(
    mut query: Query<
        (Entity, &Name, &mut AgentInteractionQueue),
        (
            With<Agent>,
            Without<Interacting>,
            // Without<WaitingInteraction>,
        ),
    >,
    mut commands: Commands,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (entity, name, mut agent_interation_queue) in &mut query {
        if !agent_interation_queue.is_empty() {
            if let Some(interaction) = agent_interation_queue.pop_first() {
                match interaction.kind {
                    AgentInteractionKind::Ask(sharing) => {
                        add_log_writer.send(AddLogEntry::new(
                            entity,
                            format!("Received Ask Interaction from {}", sharing.partner).as_str(),
                        ));
                        add_log_writer.send(AddLogEntry::new(
                            sharing.partner,
                            format!("Start Ask Interaction with target {}", name).as_str(),
                        ));
                        commands
                            .entity(sharing.partner)
                            .insert((
                                Interacting,
                                KnowledgeSharingInteraction {
                                    seller_of: sharing.seller_of,
                                    partner: entity,
                                },
                            ))
                            .remove::<WaitingInteraction>();

                        commands.entity(entity).insert((Interacting, sharing));
                    }
                    AgentInteractionKind::Trade(trade_component) => {
                        add_log_writer.send(AddLogEntry::new(
                            entity,
                            format!("Start Trade Interaction with {}", trade_component.partner)
                                .as_str(),
                        ));
                        add_log_writer.send(AddLogEntry::new(
                            trade_component.partner,
                            format!("Start Trade Interaction with {}", entity).as_str(),
                        ));
                        commands
                            .entity(trade_component.partner)
                            .insert(Interacting)
                            .remove::<WaitingInteraction>();
                        commands
                            .entity(entity)
                            .insert(TradeInteraction::new(trade_component));
                    }
                };
            }
        }
    }
}

#[derive(Component)]
struct AnimationConfig {
    first_up_index: usize,
    last_up_index: usize,
    first_left_index: usize,
    last_left_index: usize,
    first_right_index: usize,
    last_right_index: usize,
    first_down_index: usize,
    last_down_index: usize,
    fps: u8,
}

impl AnimationConfig {
    fn new() -> Self {
        Self {
            first_up_index: 0,
            last_up_index: 8,
            first_left_index: 9,
            last_left_index: 17,
            first_right_index: 27,
            last_right_index: 35,
            first_down_index: 18,
            last_down_index: 26,
            fps: 2,
        }
    }
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut shared_knowledge: ResMut<SharedKnowledge>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let texture = asset_server.load("BODY_male.png");
    let seller_texture = asset_server.load("body_dressed.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 9, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let scale = Vec3::splat(1.0);

    for _ in 0..5 {
        let entity_id = commands.spawn_empty().id();

        let v = get_random_vec3();

        commands.entity(entity_id).insert((
            Sprite {
                image: seller_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Agent::new_seller_of(ItemEnum::MEAT),
            Transform::from_scale(scale).with_translation(v),
            AnimationConfig::new(),
            AgentInteractionQueue::new(),
            Name::new("the happier seller"),
            AgentLogs::new(),
            SellerRole { location: v },
            Idle,
        ));

        shared_knowledge.add_fact(ecs::knowledge::KnowledgeFact::SellerInfo {
            entity: entity_id,
            location: v,
            wares: vec![ItemEnum::MEAT],
        });
    }

    for _ in 0..5 {
        let entity_id = commands.spawn_empty().id();

        let v = get_random_vec3();

        commands.entity(entity_id).insert((
            Sprite {
                image: seller_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Agent::new_seller_of(core::item::ItemEnum::WATER),
            Transform::from_scale(scale).with_translation(v),
            AnimationConfig::new(),
            AgentInteractionQueue::new(),
            Name::new("the happier seller"),
            AgentLogs::new(),
            SellerRole { location: v },
            Idle,
        ));

        shared_knowledge.add_fact(ecs::knowledge::KnowledgeFact::SellerInfo {
            entity: entity_id,
            location: v,
            wares: vec![ItemEnum::WATER],
        });
    }

    for i in 0..150 {
        let entity_id = commands.spawn_empty().id();

        commands.entity(entity_id).insert((
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Agent::new(),
            Transform::from_scale(scale).with_translation(Vec3::new(100., 100., 0.)),
            AnimationConfig::new(),
            AgentInteractionQueue::new(),
            AgentLogs::new(),
            Name::new(format!("agent_{}", i)),
            NoneRole,
            Idle,
        ));
    }
}

fn check_idle_agents_needs(
    query: Query<(Entity, &Agent), (With<Idle>, Without<Interacting>)>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    for (entity, agent) in &query {
        if agent.is_hungry() {
            if agent.have_food() {
                add_log_writer.send(AddLogEntry::new(entity, "Start ConsumeTask (eat)"));
                commands
                    .entity(entity)
                    .insert(ConsumeTask::new(core::item::ItemEnum::MEAT, 1))
                    .remove::<Idle>();
            } else {
                add_log_writer.send(AddLogEntry::new(entity, "Start BuyTask"));
                commands
                    .entity(entity)
                    .insert(BuyTask::new(core::item::ItemEnum::MEAT, 1))
                    .remove::<Idle>();
            }
        } else if agent.is_thirsty() {
            if agent.have_drink() {
                add_log_writer.send(AddLogEntry::new(entity, "Start ConsumeTask (drink)"));
                commands
                    .entity(entity)
                    .insert(ConsumeTask::new(core::item::ItemEnum::WATER, 1))
                    .remove::<Idle>();
            } else {
                add_log_writer.send(AddLogEntry::new(entity, "Start BuyTask (water)"));
                commands
                    .entity(entity)
                    .insert(BuyTask::new(core::item::ItemEnum::WATER, 1))
                    .remove::<Idle>();
            }
        }
    }
}

fn handle_walking_action(
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &AnimationConfig,
            &mut Sprite,
            &Walking,
        ),
        (With<Walking>, Without<Interacting>, Without<Idle>),
    >,
    time: Res<Time>,
    mut commands: Commands,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (entity, mut transform, config, mut sprite, walking) in &mut query {
        if walking.destination.distance(transform.translation) > 50. {
            let mut direction = (walking.destination - transform.translation).normalize();
            movement(&mut direction, &mut transform, &config, &mut sprite, &time);
        } else {
            add_log_writer.send(AddLogEntry::new(entity, "Walking finished with success"));
            if walking.should_set_idle_at_completion() {
                commands.entity(entity).insert(Idle).remove::<Walking>();
            } else {
                commands.entity(entity).remove::<Walking>();
            }
        }
    }
}

fn movement(
    direction: &mut Vec3,
    transform: &mut Transform,
    config: &AnimationConfig,
    sprite: &mut Sprite,
    time: &Res<Time>,
) {
    let speed = 125.0;

    if direction.length_squared() > 0.0 {
        if let Some(atlas) = &mut sprite.texture_atlas {
            if direction.y > 0.0 {
                if atlas.index >= config.last_up_index || atlas.index < config.first_up_index {
                    // ...and it IS the last frame, then we move back to the first frame and stop.
                    atlas.index = config.first_up_index;
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                }
            } else if direction.y < 0.0 {
                if atlas.index >= config.last_down_index || atlas.index < config.first_down_index {
                    // ...and it IS the last frame, then we move back to the first frame and stop.
                    atlas.index = config.first_down_index;
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                }
            } else if direction.x > 0.0 {
                if atlas.index >= config.last_right_index || atlas.index < config.first_right_index
                {
                    // ...and it IS the last frame, then we move back to the first frame and stop.
                    atlas.index = config.first_right_index;
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                }
            } else if direction.x < 0.0 {
                if atlas.index >= config.last_left_index || atlas.index < config.first_left_index {
                    // ...and it IS the last frame, then we move back to the first frame and stop.
                    atlas.index = config.first_left_index;
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                }
            }
        };

        transform.translation += direction.normalize() * speed * time.delta_secs();
    }
}
