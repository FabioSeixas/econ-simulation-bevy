mod action;
mod agent;
mod events;
mod inventory;
mod item;
mod locations;
mod role;
mod task;

use bevy::log::*;
use bevy::prelude::*;
use events::TaskEvent;

use crate::action::*;
use crate::agent::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG, // Set minimum level to show debug logs
            ..default()
        }))
        .init_resource::<Heartbeat>()
        .add_event::<TaskEvent>()
        .add_systems(Startup, setup)
        // .add_systems(Update, player_movement)
        .add_systems(Update, handle_events)
        .add_systems(Update, agent_frame)
        .add_systems(Update, heartbeat_system)
        .run();
}

#[derive(Resource)]
struct Heartbeat(Timer);

impl Default for Heartbeat {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

fn heartbeat_system(time: Res<Time>, mut heartbeat: ResMut<Heartbeat>, query: Query<&Agent>) {
    if heartbeat.0.tick(time.delta()).just_finished() {
        for agent in &query {
            if agent.role.get_name() == String::from("No Role") {
                agent.print_state();
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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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
            Agent::new(entity_id),
            Transform::from_scale(scale).with_translation(Vec3::new(100., 100., 0.)),
            AnimationConfig::new(),
        ));
    }

    for _ in 0..2 {
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
            Transform::from_scale(scale).with_translation(Vec3::new(100., 100., 0.)),
            Agent::new_seller(entity_id),
            AnimationConfig::new(),
        ));
    }

    // commands.spawn((
    //     Sprite {
    //         image: texture.clone(),
    //         texture_atlas: Some(TextureAtlas {
    //             layout: texture_atlas_layout.clone(),
    //             index: 0,
    //         }),
    //         ..default()
    //     },
    //     Transform::from_scale(scale),
    //     Player,
    //     AnimationConfig::new(),
    // ));
}

fn handle_events(mut reader: EventReader<TaskEvent>, mut query: Query<(Entity, &mut Agent)>) {
    for event in reader.read() {
        if let Ok((entity, mut agent)) = query.get_mut(event.target) {
            println!("Agent {:?} will handle {:?}", entity, event.task);
            agent.handle_event(event);
        }
    }
}

fn agent_frame(
    mut query: Query<(&mut Transform, &AnimationConfig, &mut Sprite, &mut Agent), With<Agent>>,
    mut writer: EventWriter<TaskEvent>,
    time: Res<Time>,
) {
    for (mut transform, config, mut sprite, mut agent) in &mut query {
        agent.frame_update();

        // agent.print_actions_list();

        if let Some(action) = agent.get_action() {
            match action.action_type {
                ActionType::WALK(destination) => {
                    if destination.distance(transform.translation) > 50. {
                        let mut direction = (destination - transform.translation).normalize();
                        movement(&mut direction, &mut transform, &config, &mut sprite, &time);
                    } else {
                        agent.complete_current_action()
                    }
                }
                _ => agent.do_action(&mut writer),
            }
        } else {
            agent.new_action();
        }
    }
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Transform, &AnimationConfig, &mut Sprite), With<Player>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    let (mut transform, config, mut sprite) = player.into_inner();

    movement(&mut direction, &mut transform, &config, &mut sprite, &time)
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
