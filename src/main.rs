mod core;
mod ecs;

use std::collections::VecDeque;

use bevy::log::*;
use bevy::prelude::*;

use crate::core::action::*;
use crate::core::location::Location;
use crate::ecs::agent::*;
use crate::ecs::components::*;
use crate::ecs::interaction::*;
use crate::ecs::trade::components::*;
use crate::ecs::trade::plugin::TradePlugin;
use crate::ecs::ui::plugin::UiPlugin;

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
        .add_systems(Update, update_agents_and_interrupt_system)
        .add_systems(Update, handle_idle_agents)
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
                match interaction {
                    AgentInteractionEvent::Trade(trade_component) => commands
                        .entity(entity)
                        .insert(TradeInteraction::new(trade_component)),
                };
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct KnowledgeManagement {
    seller: Vec<Entity>,
}

impl KnowledgeManagement {
    pub fn add(&mut self, entity: Entity) {
        self.seller = vec![entity];
    }

    pub fn get_knowlegde(&self) -> Entity {
        self.seller[0]
    }
}

#[derive(Component)]
pub struct AgentInteractionQueue {
    queue: VecDeque<AgentInteractionEvent>,
}

impl AgentInteractionQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, event: AgentInteractionEvent) {
        self.queue.push_back(event);
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

#[derive(Component, Default)]
pub struct Walking {
    destination: Vec3,
}

#[derive(Component, Default)]
pub struct Consuming;

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

    for _ in 0..1 {
        let entity_id = commands.spawn_empty().id();

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
            Transform::from_scale(scale).with_translation(Vec3::new(100., 100., 0.)),
            AnimationConfig::new(),
            AgentInteractionQueue::new(),
            Name::new("the happier seller"),
            Idle {},
        ));

        knowledge.add(entity_id);
    }

    for i in 0..10 {
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
            Idle {},
        ));
    }
}

fn add_marker<T: Component + Default>(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).insert(T::default());
    commands.entity(entity).remove::<Idle>();
}

fn remove_action_marker(commands: &mut Commands, entity: Entity) {
    // TODO: somehow this must give me a compile error when new actions are created
    commands
        .entity(entity)
        .remove::<(Walking, Consuming, Buying, Selling)>();
    commands.entity(entity).insert(Idle::default());
}

fn handle_idle_agents(
    mut query: Query<(Entity, &mut Agent), (With<Idle>, Without<Interacting>)>,
    mut commands: Commands,
) {
    for (entity, mut agent) in &mut query {
        if agent.get_mut_action().is_none() {
            agent.new_action();
            continue;
        }

        let action = agent.get_mut_action().unwrap();

        // println!("handle_idle_agents: {:?}", action);
        match action {
            Action::Walk(v) => {
                // println!("adding walk marker");
                commands.entity(entity).insert(Walking {
                    destination: location_to_vec3(v.get_destination()),
                });
                commands.entity(entity).remove::<Idle>();
            }
            Action::BUY(v) => {
                // println!("adding buy marker");
                commands.entity(entity).insert(Buying {
                    qty: v.qty,
                    item: v.item,
                });
                commands.entity(entity).remove::<Idle>();
            }
            Action::SELL(_) => {
                // println!("adding selling marker");
                add_marker::<Selling>(&mut commands, entity);
            }
            Action::CONSUME(_) => {
                // println!("adding consume marker");
                add_marker::<Consuming>(&mut commands, entity);
            }
        }
    }
}

fn handle_buy_action(
    mut query: Query<(Entity, &mut Agent, &Buying), Added<Buying>>,
    mut query_seller: Query<&mut AgentInteractionQueue, With<Selling>>,
    mut commands: Commands,
    knowledge: Res<KnowledgeManagement>,
) {
    for (buyer, mut agent, buying) in &mut query {
        let seller = knowledge.get_knowlegde();

        if let Ok(mut seller_agent_interaction_queue) = query_seller.get_mut(seller) {
            let buyer_trade_marker = TradeNegotiation {
                role: TradeRole::Buyer,
                quantity: buying.qty,
                item: buying.item,
                price: None,
                partner: seller,
            };
            commands.entity(buyer).insert(buyer_trade_marker);
            commands.entity(buyer).insert(Interacting);

            let seller_trade_marker = TradeNegotiation {
                role: TradeRole::Seller,
                quantity: buying.qty,
                item: buying.item,
                price: None,
                partner: buyer,
            };
            seller_agent_interaction_queue.add(AgentInteractionEvent::Trade(seller_trade_marker));
        } else {
            // seller not found
            agent.pop_current_action();
            remove_action_marker(&mut commands, buyer);
        }
    }
}

fn handle_consuming_action(
    mut query: Query<(Entity, &mut Agent), (With<Consuming>, Without<Interacting>)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut agent) in &mut query {
        let completion = if let Some(action) = agent.get_mut_action() {
            if let Action::CONSUME(consume) = action {
                match consume.current_state() {
                    ActionState::CREATED => {
                        consume.update_state();
                        None
                    }
                    ActionState::IN_PROGRESS => {
                        if consume.get_resting_duration() <= 0. {
                            consume.complete();
                        } else {
                            // println!("consuming, {:?}", consume.get_resting_duration());
                            consume.progress(time.delta_secs());
                        }
                        None
                    }
                    ActionState::COMPLETED => Some(consume.clone()),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(consume) = completion {
            let item = consume.item.clone();
            if item.is_food() {
                agent.satisfy_hungry();
            }
            agent.inventory.remove(item, consume.qty);
            agent.pop_current_action();
            remove_action_marker(&mut commands, entity);
        }
    }
}

fn handle_selling_action(
    mut query: Query<(Entity, &mut Agent), (With<Selling>, Without<Interacting>)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut agent) in &mut query {
        if let Some(action) = agent.get_mut_action() {
            if let Action::SELL(sell) = action {
                match sell.current_state() {
                    ActionState::CREATED => {
                        sell.update_state();
                    }
                    ActionState::IN_PROGRESS => {
                        if sell.get_resting_duration() <= 0. {
                            agent.pop_current_action();
                            remove_action_marker(&mut commands, entity);
                        } else {
                            // println!("happily selling, {:?}", sell.get_resting_duration());
                            sell.progress(time.delta_secs());
                        }
                    }
                    _ => {}
                }
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
            &mut Agent,
        ),
        (With<Walking>, Without<Interacting>),
    >,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, config, mut sprite, mut agent) in &mut query {
        if let Some(action) = agent.get_mut_action() {
            if let Action::Walk(walk) = action {
                let destination = location_to_vec3(walk.get_destination());
                if destination.distance(transform.translation) > 50. {
                    let mut direction = (destination - transform.translation).normalize();
                    movement(&mut direction, &mut transform, &config, &mut sprite, &time);
                } else {
                    // println!("walking done");
                    agent.pop_current_action();
                    remove_action_marker(&mut commands, entity);
                }
            }
        }
    }
}

fn location_to_vec3(location: Location) -> Vec3 {
    Vec3 {
        x: location[0],
        y: location[1],
        z: location[2],
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
