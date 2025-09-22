mod core;
mod ecs;

use std::collections::VecDeque;

use bevy::log::*;
use bevy::prelude::*;

use crate::core::item::ItemEnum;
use crate::ecs::agent::*;
use crate::ecs::components::*;
use crate::ecs::interaction::*;
use crate::ecs::knowledge::AgentKnowledge;
use crate::ecs::knowledge::KnowledgePlugin;
use crate::ecs::knowledge::SharedKnowledge;
use crate::ecs::logs::*;
use crate::ecs::roles::none::NoneRole;
use crate::ecs::roles::seller::SellerRole;
use crate::ecs::roles::{none::*, seller::*};
use crate::ecs::trade::components::*;
use crate::ecs::trade::plugin::TradePlugin;
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
        .add_plugins(UiPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                check_agent_interaction_queue_system,
                check_idle_agents_needs,
                handle_idle_sellers,
                handle_idle_none_role,
                handle_consume_task,
                handle_buy_task,
                handle_consuming_action,
                handle_selling_action,
                handle_walking_action,
                handle_buy_action,
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
        if let Ok(mut agent_memory) = agent_query.get_mut(event.target) {
            agent_memory.add(&event.description);
        }
    }
}

pub fn obtain_knowledge_system(
    mut source_agent_query: Query<
        (Entity, &Transform, &mut ObtainKnowledgeTask),
        With<ObtainKnowledgeTask>,
    >,
    mut target_agent_query: Query<(Entity, &Transform, &mut AgentInteractionQueue), With<Agent>>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    for (source_entity, source_transform, mut obtain_knowledge_task) in &mut source_agent_query {
        match obtain_knowledge_task.state {
            ObtainKnowledgeTaskState::SearchTarget => {
                let mut best: Option<(Entity, f32)> = None;
                for (entity, target_transform, _) in &target_agent_query {
                    let d2 = source_transform
                        .translation
                        .distance_squared(target_transform.translation);
                    // TODO: set a maximum acceptable distance
                    match best {
                        None => best = Some((entity, d2)),
                        Some((_, best_d2)) if d2 < best_d2 => best = Some((entity, d2)),
                        _ => {}
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
                            agent_interation_queue
                                .add(AgentInteractionKind::Ask(obtain_knowledge_task.content)),
                            closest_entity,
                        ));

                        commands
                            .entity(source_entity)
                            .insert(WaitingInteraction::new());

                        obtain_knowledge_task.state = ObtainKnowledgeTaskState::FoundTarget;
                    }
                } else {
                    // TODO
                    info!("no agents found");
                }
            }
            ObtainKnowledgeTaskState::FoundTarget => {
                if let Some((_, target_entity)) = obtain_knowledge_task.current_interaction {
                    if let Ok((_, target_transform, _)) = target_agent_query.get_mut(target_entity)
                    {
                        // TODO: check if they are already close to each other
                        commands.entity(source_entity).insert(InteractionWalking {
                            destination: target_transform.translation,
                        });
                    }

                    obtain_knowledge_task.state = ObtainKnowledgeTaskState::WalkingToTarget;
                }
            }
            ObtainKnowledgeTaskState::WalkingToTarget => {},
            ObtainKnowledgeTaskState::Interacting => {},
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
        (Entity, &mut AgentInteractionQueue),
        (
            With<Agent>,
            Without<Interacting>,
            Without<WaitingInteraction>,
        ),
    >,
    mut commands: Commands,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (entity, mut agent_interation_queue) in &mut query {
        if !agent_interation_queue.is_empty() {
            if let Some(interaction) = agent_interation_queue.pop_first() {
                match interaction.kind {
                    AgentInteractionKind::Ask(sharing) => {
                        add_log_writer.send(AddLogEntry::new(
                            entity,
                            format!("Start Ask Interaction with {}", sharing.partner).as_str(),
                        ));
                        add_log_writer.send(AddLogEntry::new(
                            sharing.partner,
                            format!("Start Ask Interaction with {}", entity).as_str(),
                        ));
                        commands
                            .entity(sharing.partner)
                            .insert((Interacting, sharing))
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
pub struct AgentInteractionQueue {
    next_id: usize,
    queue: VecDeque<AgentInteractionEvent>,
}

impl AgentInteractionQueue {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, kind: AgentInteractionKind) -> usize {
        self.queue.push_back(AgentInteractionEvent {
            id: self.next_id,
            kind,
        });
        self.next_id += 1;
        self.next_id
    }

    pub fn rm_id(&mut self, rm_id: usize) {
        self.queue.retain(|event| event.id != rm_id);
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn get_first(&mut self) -> Option<&AgentInteractionEvent> {
        match self.queue.front() {
            None => None,
            Some(v) => Some(v),
        }
    }

    pub fn pop_first(&mut self) -> Option<AgentInteractionEvent> {
        match self.queue.pop_front() {
            None => None,
            Some(v) => Some(v),
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

fn handle_buy_task(
    mut query: Query<
        (Entity, &Transform, &BuyTask, &AgentKnowledge),
        (
            With<BuyTask>,
            Without<Idle>,
            Without<Interacting>,
            Without<WaitingInteraction>,
            Without<Buying>,
            Without<Walking>,
        ),
    >,
    mut query_seller: Query<&SellerRole, With<SellerRole>>,
    mut commands: Commands,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (buyer, buyer_transform, buy_task, buyer_knowledge) in &mut query {
        let mut some_seller_found = false;

        let known_sellers = buyer_knowledge.get_sellers_of(&buy_task.item);

        if known_sellers.len() < 1 {
            add_log_writer.send(AddLogEntry::new(
                buyer,
                "Zero Known Sellers. Buy Task failed. Start ObtainKnowledgeTask",
            ));
            commands
                .entity(buyer)
                .insert(ObtainKnowledgeTask::new(KnowledgeSharing {
                    seller_of: buy_task.item,
                    partner: buyer,
                }))
                .remove::<BuyTask>();
        }

        for seller in known_sellers {
            if buy_task.tried(&seller) {
                continue;
            }

            some_seller_found = true;

            if let Ok(seller_role) = query_seller.get_mut(seller.clone()) {
                if buyer_transform.translation.distance(seller_role.location) > 50. {
                    add_log_writer.send(AddLogEntry::new(
                        buyer,
                        "Starting Walking to the seller location",
                    ));
                    let mut walking = Walking::new(seller_role.location);
                    walking.set_idle_at_completion(false);
                    commands.entity(buyer).insert(walking);
                } else {
                    add_log_writer.send(AddLogEntry::new(buyer, "Start Buying"));
                    commands
                        .entity(buyer)
                        .insert(Buying::from_buy_task(&buy_task, seller.clone()));
                }
                break;
            }
        }

        if !some_seller_found {
            add_log_writer.send(AddLogEntry::new(
                buyer,
                "No seller found. BuyTask failed. Walking around",
            ));
            commands
                .entity(buyer)
                .insert(Walking::new(get_random_vec3()))
                .remove::<BuyTask>();
        }
    }
}

fn handle_buy_action(
    mut query: Query<
        (
            Entity,
            &mut Buying,
            &mut BuyTask,
            Option<&mut WaitingInteraction>,
        ),
        (With<Buying>, Without<Idle>, Without<Interacting>),
    >,
    mut query_seller: Query<&mut AgentInteractionQueue, With<Selling>>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (buyer, mut buying, mut buy_task, waiting_interaction) in &mut query {
        if let Some(mut waiting) = waiting_interaction {
            if waiting.get_resting_duration() > 0. {
                waiting.progress(time.delta_secs());
            } else {
                if let Ok(mut seller_interaction_queue) = query_seller.get_mut(buying.seller) {
                    add_log_writer.send(AddLogEntry::new(
                        buyer,
                        "WaitingInteraction timed out, ending Buying",
                    ));
                    if buying.interaction_id.is_none() {
                        panic!("handle_buy_action, buying.interaction_id must be Some")
                    }
                    seller_interaction_queue.rm_id(buying.interaction_id.unwrap());
                    buy_task.add_tried(buying.seller);
                    commands
                        .entity(buyer)
                        .remove::<(WaitingInteraction, Buying)>();
                }
            }
        } else if let Ok(mut seller_agent_interaction_queue) = query_seller.get_mut(buying.seller) {
            add_log_writer.send(AddLogEntry::new(
                buyer,
                "Seller found, adding TradeNegotiation to the seller queue",
            ));
            let buyer_trade_marker = TradeNegotiation {
                role: TradeRole::Buyer,
                quantity: buying.qty,
                item: buying.item,
                price: None,
                partner: buying.seller.clone(),
            };
            commands
                .entity(buyer)
                .insert((buyer_trade_marker, WaitingInteraction::new()));

            let seller_trade_marker = TradeNegotiation {
                role: TradeRole::Seller,
                quantity: buying.qty,
                item: buying.item,
                price: None,
                partner: buyer,
            };

            let id = seller_agent_interaction_queue
                .add(AgentInteractionKind::Trade(seller_trade_marker));

            buying.interaction_id = Some(id);
        } else {
            add_log_writer.send(AddLogEntry::new(buyer, "Seller not found, removing Buying"));
            buy_task.add_tried(buying.seller);
            commands.entity(buyer).remove::<Buying>();
        }
    }
}

fn handle_consume_task(
    mut query: Query<
        (Entity, &Transform, &ConsumeTask),
        (
            With<ConsumeTask>,
            Without<Walking>,
            Without<Consuming>,
            Without<Idle>,
            Without<Interacting>,
        ),
    >,
    mut commands: Commands,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (entity, transform, consume_task) in &mut query {
        if consume_task.location.distance(transform.translation) > 50. {
            add_log_writer.send(AddLogEntry::new(entity, "Start Walking to consume"));
            let mut walking = Walking::new(consume_task.location);
            walking.set_idle_at_completion(false);
            commands.entity(entity).insert(walking).remove::<Idle>();
        } else {
            add_log_writer.send(AddLogEntry::new(entity, "Start Consuming"));
            commands
                .entity(entity)
                .insert(Consuming::new(consume_task.item, consume_task.qty))
                .remove::<Idle>();
        }
    }
}

fn handle_consuming_action(
    mut query: Query<
        (Entity, &mut Agent, &mut Consuming),
        (
            With<Consuming>,
            With<ConsumeTask>,
            Without<Interacting>,
            Without<Idle>,
        ),
    >,
    time: Res<Time>,
    mut commands: Commands,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (entity, mut agent, mut consuming) in &mut query {
        if consuming.get_resting_duration() > 0. {
            consuming.progress(time.delta_secs());
            continue;
        }

        let item = consuming.item.clone();
        if item.is_food() {
            add_log_writer.send(AddLogEntry::new(entity, "Consume (eat) done"));
            agent.satisfy_hungry();
        }

        if item.is_liquid() {
            add_log_writer.send(AddLogEntry::new(entity, "Consume (drink) done"));
            agent.satisfy_thirsty();
        }
        agent.inventory.remove(item, consuming.qty);

        commands
            .entity(entity)
            .insert(Idle)
            .remove::<(Consuming, ConsumeTask)>();
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

fn handle_interaction_walking_action(
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &AnimationConfig,
            &mut Sprite,
            &InteractionWalking,
        ),
        (With<InteractionWalking>, Without<Idle>),
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
            commands.entity(entity).remove::<InteractionWalking>();
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
