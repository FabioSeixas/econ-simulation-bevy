use std::usize;

use bevy::prelude::*;

use crate::action::*;
use crate::events::TaskEvent;
use crate::inventory::Inventory;
use crate::item::ItemEnum;
use crate::locations::*;
use crate::role::*;
use crate::task::*;

#[derive(Component, Debug)]
pub struct Agent {
    entity_id: Entity,
    action_system: ActionSystem,
    hungry: usize,
    eat_queued: bool,
    thirst: usize,
    drink_queued: bool,
    sleep: usize,
    sleep_queued: bool,
    inventory: Inventory,
    pub role: Box<dyn Role + Send + Sync>,
}

impl Agent {
    pub fn new_seller(entity_id: Entity) -> Self {
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
            entity_id,
        }
    }
    pub fn new(entity_id: Entity) -> Self {
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
            entity_id,
        }
    }

    pub fn print_state(&self) {
        dbg!(&self.action_system.queue);
        dbg!(&self.inventory.get_qty(ItemEnum::MONEY));
    }

    pub fn handle_event(&mut self, event: &TaskEvent) {
        match &event.task._action_type {
            ActionType::BUY((item, qty)) => {
                self.inventory.add(item.clone(), qty.clone());
                self.inventory.remove(ItemEnum::MONEY, 2);
                self.complete_current_action();
            }
            _ => {
                panic!("Not ready to handle other events than buy")
            }
        }
    }

    pub fn frame_update(&mut self) {
        self.hungry += 1;
        self.sleep += 1;
        self.thirst += 1;

        if self.hungry > NEED_THRESHOLD && !self.eat_queued {
            if self.inventory.get_qty(ItemEnum::MEAT) > 0 {
                self.action_system.new_action(ActionType::EAT);
            } else {
                self.action_system.new_action_from_task(Task {
                    _target: Some(self.entity_id),
                    _id: 1,
                    _name: String::from("buy food"),
                    _action_type: ActionType::BUY((ItemEnum::MEAT, 1)),
                    _where: get_location(NeedType::EAT),
                    _duration: 0,
                });
                self.action_system.new_action(ActionType::EAT);
            }
            self.eat_queued = true;
        }

        if self.sleep > NEED_THRESHOLD && !self.sleep_queued {
            self.action_system.new_action(ActionType::SLEEP);
            self.sleep_queued = true;
        }

        if self.thirst > NEED_THRESHOLD && !self.drink_queued {
            if self.inventory.get_qty(ItemEnum::WATER) > 0 {
                self.action_system.new_action(ActionType::DRINK);
            } else {
                self.action_system.new_action_from_task(Task {
                    _id: 1,
                    _name: String::from("buy water"),
                    _action_type: ActionType::BUY((ItemEnum::WATER, 1)),
                    _where: get_location(NeedType::DRINK),
                    _duration: 0,
                    _target: Some(self.entity_id),
                });
                self.action_system.new_action(ActionType::DRINK);
            }
            self.drink_queued = true;
        }
    }

    pub fn get_action(&mut self) -> Option<&Action> {
        self.action_system.get_action()
    }

    pub fn do_action(&mut self, writer: &mut EventWriter<TaskEvent>) {
        self.action_system.do_action(writer)
    }

    pub fn complete_current_action(&mut self) {
        if let Some(action) = self.action_system.complete_current_action() {
            match action.action_type {
                ActionType::SLEEP => {
                    self.sleep = 0;
                    self.sleep_queued = false;
                }
                ActionType::EAT => {
                    self.inventory.remove(ItemEnum::MEAT, 1);
                    self.hungry = 0;
                    self.eat_queued = false;
                }
                ActionType::DRINK => {
                    self.inventory.remove(ItemEnum::WATER, 1);
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
