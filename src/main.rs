mod core;
mod ecs;

use std::collections::VecDeque;

use bevy::log::*;
use bevy::prelude::*;

use crate::ecs::agent::*;
use crate::ecs::components::*;
use crate::ecs::interaction::*;
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
        .add_plugins(TradePlugin)
        .add_plugins(UiPlugin)
        .init_resource::<KnowledgeManagement>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_agents_and_interrupt_system, check_idle_agents_needs).chain(),
        )
        .add_systems(Update, (handle_idle_sellers, handle_idle_none_role))
        .add_systems(Update, (handle_consume_task, handle_buy_task))
        .add_systems(
            Update,
            (
                handle_consuming_action,
                handle_selling_action,
                handle_walking_action,
                handle_buy_action,
            ),
        )
        .run();
}

fn update_agents_and_interrupt_system(
    mut query: Query<
        (Entity, &mut Agent, &mut AgentInteractionQueue),
        (With<Agent>, Without<Interacting>),
    >,
    mut commands: Commands,
) {
    for (entity, mut agent, mut agent_interation_queue) in &mut query {
        agent.frame_update();

        if !agent_interation_queue.is_empty() {
            // println!(
            //     "Current agent queue size: {:?}",
            //     agent_interation_queue.len()
            // );
            if let Some(interaction) = agent_interation_queue.pop_first() {
                // println!("Pop interaction and adding to Agent: {:?}", interaction);
                match interaction.kind {
                    AgentInteractionKind::Trade(trade_component) => {
                        commands
                            .entity(trade_component.partner)
                            .insert(Interacting)
                            .remove::<WaitingInteraction>();
                        commands
                            .entity(entity)
                            .insert(TradeInteraction::new(trade_component))
                    }
                };
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct KnowledgeManagement {
    sellers: Vec<Entity>,
}

impl KnowledgeManagement {
    pub fn new() -> Self {
        Self { sellers: vec![] }
    }
    pub fn add(&mut self, entity: Entity) {
        self.sellers.push(entity);
    }

    pub fn get_sellers(&self) -> &Vec<Entity> {
        &self.sellers
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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut knowledge: ResMut<KnowledgeManagement>,
) {
    commands.spawn(Camera2d);

    let texture = asset_server.load("BODY_male.png");
    let seller_texture = asset_server.load("body_dressed.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 9, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let scale = Vec3::splat(1.0);

    for x in 0..3 {
        let entity_id = commands.spawn_empty().id();

        let v = 100. * x as f32;

        commands.entity(entity_id).insert((
            Sprite {
                image: seller_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Agent::new_seller(),
            Transform::from_scale(scale).with_translation(Vec3::new(v, v, 0.)),
            AnimationConfig::new(),
            AgentInteractionQueue::new(),
            Name::new("the happier seller"),
            SellerRole {
                location: Vec3::new(v, v, 0.),
            },
            Idle,
        ));

        knowledge.add(entity_id);
    }

    for i in 0..80 {
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
            Name::new(format!("agent_{}", i)),
            NoneRole,
            Idle,
        ));
    }
}

fn check_idle_agents_needs(
    query: Query<(Entity, &Agent), (With<Idle>, Without<Interacting>)>,
    mut commands: Commands,
) {
    for (entity, agent) in &query {
        if agent.is_hungry() {
            if agent.have_food() {
                commands
                    .entity(entity)
                    .insert(ConsumeTask::new(core::item::ItemEnum::MEAT, 1))
                    .remove::<Idle>();
            } else {
                commands
                    .entity(entity)
                    .insert(BuyTask::new(core::item::ItemEnum::MEAT, 1))
                    .remove::<Idle>();
            }
        }
    }
}

fn handle_buy_task(
    mut query: Query<
        (Entity, &Transform, &BuyTask),
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
    knowledge: Res<KnowledgeManagement>,
) {
    let sellers = knowledge.get_sellers();
    for (buyer, buyer_transform, buy_task) in &mut query {
        let mut some_seller_found = false;

        for seller in sellers {
            if buy_task.tried(seller) {
                continue;
            }

            some_seller_found = true;

            if let Ok(seller_role) = query_seller.get_mut(seller.clone()) {
                if buyer_transform.translation.distance(seller_role.location) > 50. {
                    let mut walking = Walking::new(seller_role.location);
                    walking.set_idle_at_completion(false);
                    commands.entity(buyer).insert(walking);
                } else {
                    commands
                        .entity(buyer)
                        .insert(Buying::from_buy_task(&buy_task, seller.clone()));
                }
            }
        }

        if !some_seller_found {
            println!("Tried all sellers and failed");
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
    mut commands: Commands,
    time: Res<Time>,
) {
    for (buyer, mut buying, mut buy_task, waiting_interaction) in &mut query {
        if let Some(mut waiting) = waiting_interaction {
            if waiting.get_resting_duration() > 0. {
                waiting.progress(time.delta_secs());
            } else {
                if let Ok(mut seller_interaction_queue) = query_seller.get_mut(buying.seller) {
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
            println!("Seller not found");
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
) {
    for (entity, transform, consume_task) in &mut query {
        if consume_task.location.distance(transform.translation) > 50. {
            let mut walking = Walking::new(consume_task.location);
            walking.set_idle_at_completion(false);
            commands.entity(entity).insert(walking).remove::<Idle>();
        } else {
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
) {
    for (entity, mut agent, mut consuming) in &mut query {
        if consuming.get_resting_duration() > 0. {
            consuming.progress(time.delta_secs());
            continue;
        }

        let item = consuming.item.clone();
        if item.is_food() {
            agent.satisfy_hungry();
        }
        println!("will remove food after consuming");
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
) {
    for (entity, mut transform, config, mut sprite, walking) in &mut query {
        if walking.destination.distance(transform.translation) > 50. {
            let mut direction = (walking.destination - transform.translation).normalize();
            movement(&mut direction, &mut transform, &config, &mut sprite, &time);
        } else {
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
