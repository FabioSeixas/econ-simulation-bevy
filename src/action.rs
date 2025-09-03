use std::collections::VecDeque;
use rand::Rng;
use bevy::prelude::Vec3;

use crate::locations::*;

#[derive(Debug)]
pub enum ActionType {
    WALK(Vec3),
    EAT,
    DRINK,
    SLEEP,
}

#[derive(Debug)]
pub struct Action {
    pub action_type: ActionType,
}

#[derive(Debug)]
pub struct ActionSystem {
    queue: VecDeque<Action>,
}

impl ActionSystem {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new()
        }
    }

    pub fn complete_current_action(&mut self) -> Option<Action> {
        self.queue.pop_front()
    }

    pub fn get_action(&mut self) -> Option<&Action> {
        self.queue.front()
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
                })
            }
            Some(action_type) => match action_type {
                ActionType::SLEEP => {
                    self.queue.push_front(Action {
                        action_type: ActionType::SLEEP,
                    });
                    self.queue.push_front(Action {
                        action_type: ActionType::WALK(get_location(NeedType::SLEEP)),
                    });
                }
                ActionType::EAT => {
                    self.queue.push_front(Action {
                        action_type: ActionType::EAT,
                    });
                    self.queue.push_front(Action {
                        action_type: ActionType::WALK(get_location(NeedType::EAT)),
                    });
                }
                ActionType::DRINK => {
                    self.queue.push_front(Action {
                        action_type: ActionType::DRINK,
                    });
                    self.queue.push_front(Action {
                        action_type: ActionType::WALK(get_location(NeedType::DRINK)),
                    });
                }
                ActionType::WALK(destination) => {
                    self.queue.push_front(Action {
                        action_type: ActionType::WALK(destination),
                    });
                }
            },
        }
    }
}
