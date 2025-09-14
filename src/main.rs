mod core;
mod ecs;

use std::collections::VecDeque;

use bevy::log::*;
use bevy::prelude::*;

use crate::core::action::*;
use crate::core::item::ItemEnum;
use crate::core::location::Location;
use crate::ecs::agent::*;
use crate::ecs::interaction::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG, // Set minimum level to show debug logs
            ..default()
        }))
        .add_event::<ActionCompleted>()
        .add_event::<AgentInteraction>()
        .init_resource::<KnowledgeManagement>()
        .init_resource::<OngoingInteractionsQueue>()
        .init_resource::<NewInteractionsRequests>()
        .add_systems(Startup, setup)
        .add_systems(Update, update_agents)
        .add_systems(Update, handle_idle_agents)
        .add_systems(Update, send_new_interactions_requests_to_agents)
        .add_systems(Update, handle_consuming_action)
        .add_systems(Update, handle_selling_action)
        .add_systems(Update, handle_walking_action)
        .add_systems(Update, handle_buy_action)
        .add_systems(
            Update,
            (
                process_ongoing_interaction.before(write_ongoing_interactions),
                write_ongoing_interactions,
            ),
        )
        .run();
}

fn update_agents(mut query: Query<&mut Agent, With<Agent>>) {
    for mut agent in &mut query {
        agent.frame_update();
    }
}

#[derive(Resource, Default)]
pub struct NewInteractionsRequests {
    requests: Vec<(Entity, AgentInteraction)>,
}

impl NewInteractionsRequests {
    pub fn add(&mut self, entity: Entity, interaction: AgentInteraction) {
        self.requests.push((entity, interaction))
    }

    pub fn take(&mut self) -> Vec<(Entity, AgentInteraction)> {
        std::mem::take(&mut self.requests)
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

#[derive(Event)]
struct ActionCompleted {
    pub entity: Entity,
    failed: bool,
}

impl ActionCompleted {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            failed: false,
        }
    }

    pub fn set_failed(&mut self) {
        self.failed = true
    }

    pub fn is_failed(&self) -> bool {
        self.failed
    }
}

#[derive(Component)]
pub struct AgentInteractionQueue {
    queue: VecDeque<AgentInteraction>,
    free: bool,
}

impl AgentInteractionQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            free: true,
        }
    }

    pub fn free(&mut self) {
        self.free = true;
    }

    pub fn is_free(&self) -> bool {
        self.free
    }

    pub fn add(&mut self, event: AgentInteraction) {
        self.queue.push_back(event);
    }

    pub fn pop_first(&mut self) -> Option<AgentInteraction> {
        match self.queue.pop_front() {
            None => None,
            Some(v) => {
                self.free = false;
                Some(v)
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct OngoingInteractionsQueue {
    interactions: Vec<AgentInteraction>,
}

impl OngoingInteractionsQueue {
    pub fn add(&mut self, event: AgentInteraction) {
        self.interactions.push(event);
        println!("interation added to the Ongoing Interactions Queue");
    }

    pub fn take(&mut self) -> Vec<AgentInteraction> {
        std::mem::take(&mut self.interactions)
    }
}

fn write_ongoing_interactions(
    mut interaction_writer: EventWriter<AgentInteraction>,
    mut interaction_queue: ResMut<OngoingInteractionsQueue>,
) {
    let events_to_send = interaction_queue.take();

    if events_to_send.len() > 0 {
        println!("events_to_send: {:?}", events_to_send);
    }

    for event in events_to_send {
        interaction_writer.send(event);
    }
}

fn send_new_interactions_requests_to_agents(
    mut query: Query<&mut AgentInteractionQueue, With<AgentInteractionQueue>>,
    mut new_interactions_requests: ResMut<NewInteractionsRequests>,
) {
    let new_interactions_to_send = new_interactions_requests.take();

    if new_interactions_to_send.len() > 0 {
        println!("events_to_send: {:?}", new_interactions_to_send);
    }

    for (entity, interaction) in new_interactions_to_send {
        if let Ok(mut agent_queue) = query.get_mut(entity) {
            agent_queue.add(interaction);
        }
    }
}

#[derive(Component, Default)]
pub struct Idle;

#[derive(Component, Default)]
pub struct Walking;

#[derive(Component, Default)]
pub struct Consuming;

#[derive(Component, Default)]
pub struct Buying;

#[derive(Component, Default)]
pub struct Selling;

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
            Agent::new_seller(entity_id),
            Transform::from_scale(scale).with_translation(Vec3::new(100., 100., 0.)),
            AnimationConfig::new(),
            AgentInteractionQueue::new(),
            Idle {},
        ));

        knowledge.add(entity_id);
    }

    for _ in 0..500 {
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
            Agent::new(entity_id),
            Transform::from_scale(scale).with_translation(Vec3::new(100., 100., 0.)),
            AnimationConfig::new(),
            AgentInteractionQueue::new(),
            Idle {},
        ));
    }
}

fn add_action_marker<T: Component + Default>(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).insert(T::default());
    commands.entity(entity).remove::<Idle>();
}

fn remove_action_marker<T: Component + Default>(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).remove::<T>();
    commands.entity(entity).insert(Idle::default());
}

fn handle_idle_agents(
    mut query: Query<(Entity, &mut AgentInteractionQueue, &mut Agent), With<Idle>>,
    mut commands: Commands,
    mut ongoing_interactions: ResMut<OngoingInteractionsQueue>,
) {
    for (entity, mut interation_queue, mut agent) in &mut query {
        // agent.frame_update();

        if interation_queue.is_free() {
            if let Some(v) = interation_queue.pop_first() {
                ongoing_interactions.add(v);
            }
        }

        if agent.get_mut_action().is_none() {
            agent.new_action();
            continue;
        }

        let action = agent.get_mut_action().unwrap();

        println!("handle_idle_agents: {:?}", action);
        match action {
            Action::Walk(_) => {
                println!("adding walk marker");
                add_action_marker::<Walking>(&mut commands, entity);
            }
            Action::BUY(_) => {
                println!("adding buy marker");
                add_action_marker::<Buying>(&mut commands, entity);
            }
            Action::SELL(_) => {
                println!("adding selling marker");
                add_action_marker::<Selling>(&mut commands, entity);
            }
            Action::CONSUME(_) => {
                println!("adding consume marker");
                add_action_marker::<Consuming>(&mut commands, entity);
            }
        }
    }
}

// fn action_completion(
//     mut action_completed_reader: EventReader<ActionCompleted>,
//     mut agent_query: Query<&mut Agent>,
// ) {
//     for event in action_completed_reader.read() {
//         if agent_query.get_mut(event.entity).is_err() {
//             warn!(
//                 "ActionCompleted event for entity {:?}, but it has no Agent component!",
//                 event.entity
//             );
//             continue;
//         }
//
//         let mut agent = agent_query.get_mut(event.entity).unwrap();
//
//         if agent.get_action().is_none() {
//             continue;
//         }
//
//         let action = agent.get_action().cloned().unwrap();
//         match action {
//             Action::Walk(_) => {
//                 agent.pop_current_action();
//             }
//             Action::SELL(_) => {
//                 agent.pop_current_action();
//             }
//             Action::BUY(v) => {
//                 buy_action_callback(&mut agent, &v, event.is_failed());
//             }
//             Action::CONSUME(v) => {
//                 consume_action_callback(&mut agent, &v, event.is_failed());
//             }
//         }
//     }
// }

fn process_ongoing_interaction(
    mut interaction_reader: EventReader<AgentInteraction>,
    mut interaction_queue: ResMut<OngoingInteractionsQueue>,
    mut action_completed_writer: EventWriter<ActionCompleted>,
    mut agent_query: Query<(&mut Agent, &mut AgentInteractionQueue)>,
) {
    for event in interaction_reader.read() {
        if agent_query.get_mut(event.target).is_err() {
            warn!(
                "ActionCompleted event for entity {:?}, but it has no Agent component!",
                event.target
            );
            continue;
        }

        println!("Start event {:?}", event);

        let (mut target_agent, mut agent_interaction_queue) =
            agent_query.get_mut(event.target).unwrap();

        // if event.is_failed() {
        //     if let Some(_) = target_agent.get_mut_action() {
        //         let mut completion = ActionCompleted::new(event.target);
        //         completion.set_failed();
        //         action_completed_writer.send(completion);
        //     }
        //
        //     agent_interaction_queue.free();
        //     continue;
        // }

        if let Some(trade) = &event.trade {
            if let Some(action) = target_agent.get_mut_action() {
                if let Action::SELL(sell_action) = action {
                    match trade.get_status() {
                        TradeStatus::NEGOTIATION => {
                            println!("Seller received {:?}", event);
                            sell_action.update_state();

                            let mut updated_trade = trade.clone();
                            let seller_amount = target_agent.inventory.get_qty(trade.item);

                            if seller_amount == 0 {
                                // todo: complete action with failed
                                continue;
                            }

                            if seller_amount < trade.qty {
                                updated_trade.qty = seller_amount;
                            }
                            updated_trade.price = Some(updated_trade.qty * 3);

                            interaction_queue.add(AgentInteraction::new_with_trade(
                                event.target,
                                event.source,
                                Some(updated_trade),
                            ));
                        }
                        TradeStatus::DONE => {
                            println!("Seller received {:?}", event);
                            if trade.price.is_none() {
                                panic!("Trade status DONE without price")
                            }

                            sell_action.update_state();

                            target_agent.inventory.remove(trade.item, trade.qty);
                            target_agent
                                .inventory
                                .add(core::item::ItemEnum::MONEY, trade.price.unwrap());

                            interaction_queue.add(AgentInteraction::new_with_trade(
                                event.target,
                                event.source,
                                Some(trade.clone()),
                            ));

                            agent_interaction_queue.free();
                        }
                    }
                } else if let Action::BUY(buy_action) = action {
                    match trade.get_status() {
                        TradeStatus::NEGOTIATION => {
                            println!("Buyer received {:?}", event);
                            if event.is_failed() {
                                buy_action.failed();
                                println!("buy action failed");
                                continue;
                            }

                            if trade.price.is_none() {
                                panic!("Buyer do not received price from seller")
                            }

                            let mut updated_trade = trade.clone();
                            // TODO: handle when buyer do not have enought money. Decrease qty.

                            updated_trade.buyer_accepted();
                            buy_action.set_price_paid(trade.price.unwrap());

                            interaction_queue.add(AgentInteraction::new_with_trade(
                                event.target,
                                event.source,
                                Some(updated_trade),
                            ))
                        }
                        TradeStatus::DONE => {
                            println!("Buyer received {:?}", event);
                            buy_action.price_paid = trade.price;
                            buy_action.complete();
                            // action_completed_writer.send(ActionCompleted::new(event.target));
                            agent_interaction_queue.free();
                        }
                    }
                } else {
                    println!("trade event arrive but target agent is neither sell or buy");
                    let mut interaction_feedback =
                        AgentInteraction::new(event.target, event.source);
                    interaction_feedback.set_failed();
                    interaction_queue.add(interaction_feedback);
                    agent_interaction_queue.free();
                }
            }
        }
    }
}

fn handle_buy_action(
    mut query: Query<(Entity, &mut Agent), With<Buying>>,
    mut commands: Commands,
    knowledge: Res<KnowledgeManagement>,
    mut ongoing_interactions: ResMut<OngoingInteractionsQueue>,
) {
    for (entity, mut agent) in &mut query {
        println!("consuming arrived");
        let action_state_and_data = if let Some(action) = agent.get_action() {
            if let Action::BUY(buy) = action {
                Some((
                    buy.current_state(),
                    buy.item.clone(),
                    buy.qty,
                    buy.price_paid,
                ))
            } else {
                None
            }
        } else {
            None
        };

        println!("action_state_and_data: {:?}", action_state_and_data);

        if let Some((state, item, qty, price_paid)) = action_state_and_data {
            match state {
                ActionState::CREATED => {
                    if let Some(Action::BUY(buy)) = agent.get_mut_action() {
                        buy.update_state();
                        let seller = knowledge.get_knowlegde();
                        let mut interaction = AgentInteraction::new(entity, seller);
                        interaction.trade = Some(Trade::new(&item, qty));
                        ongoing_interactions.add(interaction);
                    }
                }
                ActionState::COMPLETED => {
                    if price_paid.is_none() {
                        panic!("callback called without price paid")
                    }

                    agent.inventory.add(item.clone(), qty);
                    agent.inventory.remove(ItemEnum::MONEY, price_paid.unwrap());

                    agent.pop_current_action();
                    remove_action_marker::<Buying>(&mut commands, entity);
                }
                ActionState::FAILED => {
                    // this will make the agent try again
                    agent.pop_current_action();
                    remove_action_marker::<Buying>(&mut commands, entity);
                }
                _ => {}
            }
        }
    }
}

fn handle_consuming_action(
    mut query: Query<(Entity, &mut Agent), With<Consuming>>,
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
                            println!("consuming, {:?}", consume.get_resting_duration());
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
            remove_action_marker::<Consuming>(&mut commands, entity);
        }
    }
}

fn handle_selling_action(
    mut query: Query<(Entity, &mut Agent), With<Selling>>,
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
                            remove_action_marker::<Selling>(&mut commands, entity);
                        } else {
                            println!("happily selling, {:?}", sell.get_resting_duration());
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
        With<Walking>,
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
                    println!("walking done");
                    agent.pop_current_action();
                    remove_action_marker::<Walking>(&mut commands, entity);
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
    let speed = 150.0;

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
