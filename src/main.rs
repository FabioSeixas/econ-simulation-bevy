use std::collections::VecDeque;

use bevy::log::*;
use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG, // Set minimum level to show debug logs
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .add_systems(Update, agent_frame)
        .run();
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
struct Agent {
    action_system: ActionSystem,
    // destination: Option<Vec3>,
}

impl Agent {
    fn new() -> Self {
        Self {
            // destination: Option::None,
            action_system: ActionSystem {
                queue: VecDeque::new(),
            },
        }
    }

    fn get_action(&self) -> Option<&Action> {
        self.action_system.queue.front()
    }

    fn complete_current_action(&mut self) {
        self.action_system.queue.pop_front();
    }

    fn new_action(&mut self) {
        self.action_system.new_action();
    }
}

enum ActionType {
    WALK(Vec3),
}

struct Action {
    _type: ActionType,
}

struct ActionSystem {
    queue: VecDeque<Action>,
}

impl ActionSystem {
    fn new_action(&mut self) {
        let mut rnd = rand::thread_rng();
        let max = 500.;

        self.queue.push_back(Action {
            _type: ActionType::WALK(Vec3 {
                x: rnd.gen_range(-max..max),
                y: rnd.gen_range(-max..max),
                z: 0.,
            }),
        });
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

    for _ in 0..50 {
        commands.spawn((
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_scale(scale).with_translation(Vec3::new(100., 100., 0.)),
            Agent::new(),
            AnimationConfig::new(),
        ));
    }

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_scale(scale),
        Player,
        AnimationConfig::new(),
    ));
}

fn agent_frame(
    mut query: Query<(&mut Transform, &AnimationConfig, &mut Sprite, &mut Agent), With<Agent>>,
    time: Res<Time>,
) {
    for (mut transform, config, mut sprite, mut agent) in &mut query {
        if let Some(action) = agent.get_action() {
            match action._type {
                ActionType::WALK(destination) => {
                    if destination.distance(transform.translation) > 50. {
                        let mut direction = (destination - transform.translation).normalize();
                        movement(&mut direction, &mut transform, &config, &mut sprite, &time);
                    } else {
                        agent.complete_current_action()
                    }
                }
            }
        } else {
            agent.new_action();
        }
    }
}

// fn agent_movement(
//     mut query: Query<(&mut Transform, &AnimationConfig, &mut Sprite, &mut Agent), With<Agent>>,
//     time: Res<Time>,
// ) {
//     let mut direction = Vec3::ZERO;
//
//     for (mut transform, config, mut sprite, mut agent) in &mut query {
//         if let Some(dest) = agent.destination {
//             if dest.distance(transform.translation) > 50. {
//                 direction = (dest - transform.translation).normalize();
//             } else {
//                 agent.destination = Option::None;
//             }
//         } else {
//             let mut rnd = rand::thread_rng();
//             let max = 500.;
//
//             agent.destination = Some(Vec3 {
//                 x: rnd.gen_range(-max..max),
//                 y: rnd.gen_range(-max..max),
//                 z: 0.,
//             });
//         }
//
//         movement(&mut direction, &mut transform, &config, &mut sprite, &time)
//     }
// }

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
