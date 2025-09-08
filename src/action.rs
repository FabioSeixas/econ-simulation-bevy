use bevy::prelude::{EventWriter, Vec3};
use std::collections::VecDeque;

use crate::{events::*, item::ItemEnum, locations::*, task::Task};

#[derive(Debug, Clone, PartialEq)]
pub enum ActionType {
    WALK(Vec3),
    EAT,
    DRINK,
    SLEEP,
    WORK,
    BUY((ItemEnum, usize)),
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::SLEEP
    }
}

#[derive(Debug)]
enum CompletionCategory {
    DURATION,
    INTERACTION,
}

impl Default for CompletionCategory {
    fn default() -> Self {
        CompletionCategory::DURATION
    }
}

#[derive(Debug)]
enum ActionStatus {
    CREATED,
    WAITING,
    COMPLETED,
}

#[derive(Debug, Default)]
pub struct Action {
    pub action_type: ActionType,
    resting_duration: u32,
    complete_category: CompletionCategory,
    source_task: Option<Task>,
    status: Option<ActionStatus>,
}

#[derive(Debug, Default)]
pub struct ActionSystem {
    pub queue: VecDeque<Action>,
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

    pub fn do_action(&mut self, writer: &mut EventWriter<TaskEvent>) {
        if let Some(action) = self.queue.front_mut() {
            match action.complete_category {
                CompletionCategory::DURATION => {
                    if action.resting_duration <= 0 {
                        self.complete_current_action();
                        return;
                    }

                    action.resting_duration -= 2;
                }
                CompletionCategory::INTERACTION => {
                    if let Some(status) = &action.status {
                        match status {
                            ActionStatus::CREATED => {
                                if let Some(task) = &action.source_task {
                                    writer.send(TaskEvent {
                                        target: task._target.unwrap(),
                                        task: task.clone(),
                                        status: TaskEventStatus::COMPLETED,
                                    });
                                }
                                action.status = Some(ActionStatus::WAITING);
                            }
                            ActionStatus::WAITING => {
                                return;
                            }
                            ActionStatus::COMPLETED => {
                                self.complete_current_action();
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn new_action_from_task(&mut self, task: Task) {
        let duration = task._duration;
        match &task._action_type {
            ActionType::BUY(value) => {
                let walk = Action {
                    action_type: ActionType::WALK(task._where),
                    source_task: None,
                    ..Default::default()
                };
                let buy = Action {
                    action_type: ActionType::BUY(value.clone()),
                    source_task: Some(task),
                    resting_duration: 0,
                    complete_category: CompletionCategory::INTERACTION,
                    status: Some(ActionStatus::CREATED),
                };
                self.queue.push_back(walk);
                self.queue.push_back(buy);
            }
            ActionType::WORK => {
                let walk = Action {
                    action_type: ActionType::WALK(task._where),
                    source_task: None,
                    ..Default::default()
                };
                let work = Action {
                    action_type: ActionType::WORK,
                    source_task: Some(task),
                    resting_duration: duration,
                    complete_category: CompletionCategory::DURATION,
                    status: None,
                };
                self.queue.push_back(walk);
                self.queue.push_back(work);
            }
            ActionType::WALK(_) => {
                let walk = Action {
                    action_type: ActionType::WALK(task._where),
                    source_task: None,
                    ..Default::default()
                };
                self.queue.push_back(walk);
            }
            _ => {
                panic!(
                    "new_action_from_task Not ready to handle this action {:?}",
                    task._action_type
                )
            }
        }
    }

    pub fn new_action(&mut self, action_type: ActionType) {
        match action_type {
            ActionType::SLEEP => {
                self.queue.push_back(Action {
                    action_type: ActionType::WALK(get_location(NeedType::SLEEP)),
                    ..Default::default()
                });
                self.queue.push_back(Action {
                    action_type: ActionType::SLEEP,
                    ..Default::default()
                });
            }
            ActionType::EAT => {
                self.queue.push_back(Action {
                    action_type: ActionType::EAT,
                    ..Default::default()
                });
                // self.queue.push_front(Action {
                //     action_type: ActionType::WALK(get_location(NeedType::EAT)),
                //     ..Default::default()
                // });
            }
            ActionType::DRINK => {
                self.queue.push_back(Action {
                    action_type: ActionType::DRINK,
                    ..Default::default()
                });
                // self.queue.push_front(Action {
                //     action_type: ActionType::WALK(get_location(NeedType::DRINK)),
                //     ..Default::default()
                // });
            }
            ActionType::WALK(destination) => {
                self.queue.push_back(Action {
                    action_type: ActionType::WALK(destination),
                    ..Default::default()
                });
            }
            _ => {
                panic!(
                    "new_action Not ready to handle this action {:?}",
                    action_type
                )
            }
        }
    }
}
