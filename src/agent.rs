use bevy::prelude::*;

use crate::action::*;
use crate::inventory::Inventory;
use crate::locations::*;
use crate::role::*;

#[derive(Component, Debug)]
pub struct Agent {
    action_system: ActionSystem,
    hungry: usize,
    eat_queued: bool,
    thirst: usize,
    drink_queued: bool,
    sleep: usize,
    sleep_queued: bool,
    inventory: Inventory,
    role: Box<dyn Role + Send + Sync>,
}

impl Agent {
    pub fn new_seller() -> Self {
        Self {
            hungry: 0,
            thirst: 0,
            sleep: 0,
            eat_queued: false,
            sleep_queued: false,
            drink_queued: false,
            action_system: ActionSystem::new(),
            inventory: Inventory::new_seller(),
            role: get_seller_role(),
        }
    }
    pub fn new() -> Self {
        Self {
            hungry: 0,
            thirst: 0,
            sleep: 0,
            eat_queued: false,
            sleep_queued: false,
            drink_queued: false,
            action_system: ActionSystem::new(),
            inventory: Inventory::new(),
            role: get_random_role(),
        }
    }

    pub fn frame_update(&mut self) -> Option<&Action> {
        self.hungry += 1;
        self.sleep += 1;
        self.thirst += 1;

        if self.hungry > NEED_THRESHOLD && !self.eat_queued {
            self.action_system.new_action(ActionType::EAT);
            self.eat_queued = true;
        }

        if self.sleep > NEED_THRESHOLD && !self.sleep_queued {
            self.action_system.new_action(ActionType::SLEEP);
            self.sleep_queued = true;
        }

        if self.thirst > NEED_THRESHOLD && !self.drink_queued {
            self.action_system.new_action(ActionType::DRINK);
            self.drink_queued = true;
        }

        self.action_system.get_action()
    }

    pub fn get_action(&mut self) -> Option<&Action> {
        self.action_system.get_action()
    }

    pub fn do_action(&mut self) {
        self.action_system.do_action()
    }

    pub fn complete_current_action(&mut self) {
        if let Some(action) = self.action_system.complete_current_action() {
            match action.action_type {
                ActionType::SLEEP => {
                    self.sleep = 0;
                    self.sleep_queued = false;
                }
                ActionType::EAT => {
                    self.hungry = 0;
                    self.eat_queued = false;
                }
                ActionType::DRINK => {
                    self.thirst = 0;
                    self.drink_queued = false;
                }
                _ => {}
            }
        }
    }

    pub fn new_action(&mut self) {
        let mut task = Task::new();
        self.role.get_next_task(&mut task);
        self.action_system.new_action_from_task(task);
    }
}
