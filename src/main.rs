mod core;
mod ecs;

use bevy::log::*;
use bevy::prelude::*;

use crate::core::item::ItemEnum;
use crate::ecs::agent::*;
use crate::ecs::buy::plugin::BuyPlugin;
use crate::ecs::buy::tasks::components::BuyTask;
use crate::ecs::components::*;
use crate::ecs::consume::plugin::ConsumePlugin;
use crate::ecs::consume::tasks::components::ConsumeTask;
use crate::ecs::interaction::*;
use crate::ecs::knowledge::KnowledgePlugin;
use crate::ecs::knowledge::SharedKnowledge;
use crate::ecs::logs::*;
use crate::ecs::roles::none::NoneRole;
use crate::ecs::roles::plugin::RolesPlugin;
use crate::ecs::roles::seller::SellerRole;
use crate::ecs::sell::plugin::SellPlugin;
use crate::ecs::talk::interaction::components::KnowledgeSharingInteraction;
use crate::ecs::talk::plugin::TalkPlugin;
use crate::ecs::trade::components::*;
use crate::ecs::trade::plugin::TradePlugin;
use crate::ecs::ui::plugin::UiPlugin;
use crate::ecs::utils::get_random_vec3;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    Running,
    Paused,
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if *state == GameState::Running {
            next_state.set(GameState::Paused);
        } else {
            next_state.set(GameState::Running);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG, // Set minimum level to show debug logs
            ..default()
        }))
        .init_state::<GameState>()
        .add_event::<AddLogEntry>()
        .add_plugins(TradePlugin)
        .add_plugins(TalkPlugin)
        .add_plugins(ConsumePlugin)
        .add_plugins(KnowledgePlugin)
        .add_plugins(RolesPlugin)
        .add_plugins(SellPlugin)
        .add_plugins(BuyPlugin)
        .add_plugins(UiPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            First,
            (update_agents, check_idle_agents_needs)
                .chain()
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(
            PreUpdate,
            check_agent_interaction_queue_system.run_if(in_state(GameState::Running)),
        )
        .add_systems(
            Update,
            (handle_walking_action, add_logs_system)
                .chain()
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(Last, toggle_pause)
        .run();
}

pub fn add_logs_system(
    mut agent_query: Query<&mut AgentLogs>,
    mut add_logs_reader: EventReader<AddLogEntry>,
) {
    for event in add_logs_reader.read() {
        if let Ok(mut agent_logs) = agent_query.get_mut(event.target) {
            agent_logs.add(&event.description);
        }
    }
}

fn update_agents(mut query: Query<&mut Agent>) {
    for mut agent in &mut query {
        agent.frame_update();
    }
}

fn check_agent_interaction_queue_system(
    mut query: Query<
        (Entity, &Name, &mut AgentInteractionQueue),
        (Without<Interacting>, Without<WaitingInteraction>),
    >,
    mut commands: Commands,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (target_entity, target_name, mut agent_interation_queue) in &mut query {
        if !agent_interation_queue.is_empty() {
            if let Some(interaction) = agent_interation_queue.pop_first() {
                match interaction.kind {
                    AgentInteractionKind::Ask(sharing) => {
                        add_log_writer.send(AddLogEntry::new(
                            target_entity,
                            format!("Received Ask Interaction from {}", sharing.partner_name)
                                .as_str(),
                        ));
                        add_log_writer.send(AddLogEntry::new(
                            sharing.source,
                            format!("Start Ask Interaction with target {}", target_name).as_str(),
                        ));

                        commands
                            .entity(sharing.source)
                            .insert(KnowledgeSharingInteraction::new(
                                sharing.seller_of,
                                sharing.source,
                                target_entity,
                                target_name.clone(),
                            ))
                            .remove::<WaitingInteraction>();

                        commands.entity(target_entity).insert(sharing);
                    }
                    AgentInteractionKind::Trade(trade_component) => {
                        add_log_writer.send(AddLogEntry::new(
                            target_entity,
                            format!("Start Trade Interaction with {}", trade_component.partner)
                                .as_str(),
                        ));
                        add_log_writer.send(AddLogEntry::new(
                            trade_component.partner,
                            format!("Start Trade Interaction with {}", target_entity).as_str(),
                        ));

                        commands
                            .entity(trade_component.partner)
                            .insert(Interacting)
                            .remove::<WaitingInteraction>();
                        commands
                            .entity(target_entity)
                            .insert(TradeInteraction::new(trade_component));
                    }
                };
                // this will start only ONE interaction by frame
                // and avoid the same agent start two interactions in the same frame
                break;
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

    for i in 0..5 {
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
            Name::new(format!("the happier meat seller {}", i)),
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

    for i in 0..5 {
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
            Name::new(format!("the happier water seller {}", i)),
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
    query: Query<(Entity, &Agent, &Idle)>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    for (entity, agent, _) in &query {
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
    mut query: Query<(
        Entity,
        &mut Transform,
        &AnimationConfig,
        &mut Sprite,
        &Walking,
    )>,
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
