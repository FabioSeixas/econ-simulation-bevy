use bevy::prelude::Vec3;
use rand::Rng;
use std::collections::VecDeque;

use crate::{locations::*, role::Task};

#[derive(Debug)]
pub enum ActionType {
    WALK(Vec3),
    EAT,
    DRINK,
    SLEEP,
    WORK,
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::SLEEP // pick a sensible default
    }
}

#[derive(Debug, Default)]
pub struct Action {
    pub action_type: ActionType,
    source_task: Option<Task>,
}

#[derive(Debug, Default)]
pub struct ActionSystem {
    queue: VecDeque<Action>,
}

impl ActionSystem {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn complete_current_action(&mut self) -> Option<Action> {
        self.queue.pop_front()
    }

    pub fn get_action(&mut self) -> Option<&Action> {
        self.queue.front()
    }

    pub fn new_action_from_task(&mut self, task: Task) {
        let walk = Action {
            action_type: ActionType::WALK(task._where),
            source_task: None,
        };
        let work = Action {
            action_type: ActionType::WORK,
            source_task: Some(task),
        };
        self.queue.push_front(work);
        self.queue.push_front(walk);
    }

    pub fn new_action(&mut self, action: Option<ActionType>) {
        match action {
            None => {
                let mut rnd = rand::thread_rng();
                let max = 500.;

                self.queue.push_back(Action {
                    action_type: ActionType::WALK(Vec3 {
                        x: rnd.gen_range(-max..max),
                        y: rnd.gen_range(-max..max),
                        z: 0.,
                    }),
                    ..Default::default()
                })
            }
            Some(action_type) => match action_type {
                ActionType::SLEEP => {
                    self.queue.push_front(Action {
                        action_type: ActionType::SLEEP,
                        ..Default::default()
                    });
                    self.queue.push_front(Action {
                        action_type: ActionType::WALK(get_location(NeedType::SLEEP)),
                        ..Default::default()
                    });
                }
                ActionType::EAT => {
                    self.queue.push_front(Action {
                        action_type: ActionType::EAT,
                        ..Default::default()
                    });
                    self.queue.push_front(Action {
                        action_type: ActionType::WALK(get_location(NeedType::EAT)),
                        ..Default::default()
                    });
                }
                ActionType::DRINK => {
                    self.queue.push_front(Action {
                        action_type: ActionType::DRINK,
                        ..Default::default()
                    });
                    self.queue.push_front(Action {
                        action_type: ActionType::WALK(get_location(NeedType::DRINK)),
                        ..Default::default()
                    });
                }
                ActionType::WALK(destination) => {
                    self.queue.push_front(Action {
                        action_type: ActionType::WALK(destination),
                        ..Default::default()
                    });
                }
                _ => {
                    panic!("Not ready to handle this action")
                }
            },
        }
    }
}
