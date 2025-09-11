mod core;
mod ecs;

use bevy::log::*;
use bevy::prelude::*;
use ecs::action::{buy_action_callback, consume_action_callback};

use crate::core::action::*;
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
        .init_resource::<InteractionQueue>()
        .add_systems(Startup, setup)
        .add_systems(Update, (agent_frame, movement_frame, event_completion))
        .add_systems(
            Update,
            (
                event_interaction.before(write_outgoing_interactions),
                write_outgoing_interactions,
            ),
        )
        .run();
}

// type WhereToBuyKnowledge = (Entity, Vec3);

#[derive(Resource, Default)]
pub struct KnowledgeManagement {
    // knowledge: HashMap<Entity, Vec<WhereToBuyKnowledge>>,
    seller: Vec<Entity>,
}

impl KnowledgeManagement {
    // pub fn add(&mut self, entity: Entity, item: WhereToBuyKnowledge) {
    //     match self.knowledge.get_mut(&entity) {
    //         None => {
    //             self.knowledge.insert(entity, vec![item]);
    //         }
    //         Some(v) => v.push(item),
    //     }
    // }
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
}

#[derive(Resource, Default)]
pub struct InteractionQueue {
    pub interactions: Vec<AgentInteraction>,
}

impl InteractionQueue {
    pub fn add(&mut self, event: AgentInteraction) {
        self.interactions.push(event);
        println!("interation added to the qeueue");
    }

    pub fn take(&mut self) -> Vec<AgentInteraction> {
        std::mem::take(&mut self.interactions)
    }
}

fn write_outgoing_interactions(
    mut interaction_writer: EventWriter<AgentInteraction>,
    mut interaction_queue: ResMut<InteractionQueue>,
) {
    let events_to_send = interaction_queue.take();
    println!("events_to_send: {:?}", events_to_send);
    for event in events_to_send {
        interaction_writer.send(event);
    }
}

#[derive(Component)]
pub struct Walking {
    pub destination: Vec3,
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
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 9, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let scale = Vec3::splat(1.0);

    for _ in 0..1 {
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
            Agent::new_seller(entity_id),
            Transform::from_scale(scale).with_translation(Vec3::new(100., 100., 0.)),
            AnimationConfig::new(),
        ));

        knowledge.add(entity_id);
    }

    for _ in 0..1 {
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
        ));
    }
}

fn agent_frame(
    mut query: Query<(Entity, &mut Agent), With<Agent>>,
    mut commands: Commands,
    mut action_completed_writer: EventWriter<ActionCompleted>,
    mut interaction_writer: EventWriter<AgentInteraction>,
    time: Res<Time>,
    knowledge: Res<KnowledgeManagement>,
) {
    for (entity, mut agent) in &mut query {
        agent.frame_update();

        if agent.get_mut_action().is_none() {
            agent.new_action();
            continue;
        }

        let action = agent.get_mut_action().unwrap();
        match action {
            Action::Walk(v) => match v.current_state() {
                // ActionState::COMPLETED => {
                //     agent.complete_current_action();
                // }
                // ActionState::IN_PROGRESS => {}
                // ActionState::WAITING => {}
                ActionState::CREATED => {
                    v.update_state();
                    let destination = v.get_destination();

                    commands.entity(entity).insert(Walking {
                        destination: Vec3 {
                            x: destination[0],
                            y: destination[1],
                            z: destination[2],
                        },
                    });
                }
                _ => {}
            },
            Action::BUY(v) => match v.current_state() {
                // ActionState::COMPLETED => {
                //     buy_action_callback(agent, *v);
                // }
                // ActionState::IN_PROGRESS => {}
                // ActionState::WAITING => {}
                ActionState::CREATED => {
                    v.update_state();
                    let seller = knowledge.get_knowlegde();
                    interaction_writer.send(AgentInteraction {
                        source: entity,
                        target: seller,
                        trade: Some(Trade::new(&v.item, v.qty)),
                    });
                }
                _ => {}
            },
            Action::SELL(v) => match v.current_state() {
                ActionState::COMPLETED => {}
                ActionState::IN_PROGRESS => {
                    if v.get_resting_duration() <= 0. {
                        action_completed_writer.send(ActionCompleted { entity });
                        v.complete();
                    } else {
                        println!("happily selling, {:?}", v.get_resting_duration());
                        v.progress(time.delta_secs());
                    }
                }
                ActionState::WAITING => {}
                ActionState::CREATED => {
                    v.update_state();
                }
                _ => {}
            },
            Action::CONSUME(v) => match v.current_state() {
                ActionState::COMPLETED => {}
                ActionState::IN_PROGRESS => {
                    if v.get_resting_duration() <= 0. {
                        action_completed_writer.send(ActionCompleted { entity });
                        v.complete();
                    } else {
                        println!("consuming, {:?}", v.get_resting_duration());
                        v.progress(time.delta_secs());
                    }
                }
                ActionState::WAITING => {}
                ActionState::CREATED => {
                    v.update_state();
                }
                _ => {}
            },
        }
    }
}

fn event_completion(
    mut action_completed_reader: EventReader<ActionCompleted>,
    mut agent_query: Query<&mut Agent>,
) {
    for event in action_completed_reader.read() {
        if agent_query.get_mut(event.entity).is_err() {
            warn!(
                "ActionCompleted event for entity {:?}, but it has no Agent component!",
                event.entity
            );
            continue;
        }

        let mut agent = agent_query.get_mut(event.entity).unwrap();

        if agent.get_action().is_none() {
            continue;
        }

        let action = agent.get_action().cloned().unwrap();
        match action {
            Action::Walk(_) => {
                agent.complete_current_action();
            }
            Action::SELL(_) => {
                agent.complete_current_action();
            }
            Action::BUY(v) => {
                buy_action_callback(&mut agent, &v);
                agent.complete_current_action();
            }
            Action::CONSUME(v) => {
                consume_action_callback(&mut agent, &v);
                agent.complete_current_action();
            }
        }
    }
}

fn event_interaction(
    mut interaction_reader: EventReader<AgentInteraction>,
    mut interaction_queue: ResMut<InteractionQueue>,
    mut action_completed_writer: EventWriter<ActionCompleted>,
    mut agent_query: Query<&mut Agent>,
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

        let mut target_agent = agent_query.get_mut(event.target).unwrap();

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

                            interaction_queue.add(AgentInteraction {
                                source: event.target,
                                target: event.source,
                                trade: Some(updated_trade),
                            });
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

                            interaction_queue.add(AgentInteraction {
                                source: event.target,
                                target: event.source,
                                trade: Some(trade.clone()),
                            });
                        }
                    }
                } else if let Action::BUY(buy_action) = action {
                    match trade.get_status() {
                        TradeStatus::NEGOTIATION => {
                            println!("Buyer received {:?}", event);
                            if trade.price.is_none() {
                                panic!("Buyer do not received price from seller")
                            }

                            let mut updated_trade = trade.clone();
                            // TODO: handle when buyer do not have enought money. Decrease qty.

                            updated_trade.buyer_accepted();
                            buy_action.set_price_paid(trade.price.unwrap());

                            interaction_queue.add(AgentInteraction {
                                source: event.target,
                                target: event.source,
                                trade: Some(updated_trade),
                            });
                        }
                        TradeStatus::DONE => {
                            println!("Buyer received {:?}", event);
                            buy_action.price_paid = trade.price;
                            action_completed_writer.send(ActionCompleted {
                                entity: event.target,
                            });
                        }
                    }
                }
            }
        }
    }
}

fn movement_frame(
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &AnimationConfig,
            &mut Sprite,
            // &mut Agent,
            &Walking,
        ),
        With<Walking>,
    >,
    time: Res<Time>,
    mut commands: Commands,
    mut action_completed_writer: EventWriter<ActionCompleted>,
) {
    for (entity, mut transform, config, mut sprite, walking) in &mut query {
        if walking.destination.distance(transform.translation) > 50. {
            let mut direction = (walking.destination - transform.translation).normalize();
            movement(&mut direction, &mut transform, &config, &mut sprite, &time);
        } else {
            println!("action done");
            commands.entity(entity).remove::<Walking>();
            action_completed_writer.send(ActionCompleted { entity });
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
