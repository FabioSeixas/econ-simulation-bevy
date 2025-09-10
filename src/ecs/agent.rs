use crate::core::{
    action::*,
    inventory::*,
    item::ItemEnum,
    needs::*,
    role::{get_random_role, Role},
    task::*,
};
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
pub struct Agent {
    pub entity_id: Entity,
    pub needs: Needs,
    pub inventory: Inventory,
    queue: VecDeque<Action>,
    pub role: Box<dyn Role + Send + Sync>,
}

impl Agent {
    pub fn new(entity_id: Entity) -> Self {
        Self {
            needs: Needs::new(),
            inventory: Inventory::new(),
            queue: VecDeque::new(),
            entity_id,
            role: get_random_role(),
        }
    }

    pub fn complete_current_action(&mut self) {
        let completed_action = self.queue.pop_front();
        println!("action completed. {:?}", completed_action);
        println!("Current Inventory: {:?}", self.inventory);
    }

    pub fn satisfy_hungry(&mut self) {
        self.needs.satisfy_hunger();
    }

    pub fn new_action(&mut self) {
        if self.needs.is_hungry() {
            return self.deal_with_hungry();
        }

        self.role.calculate_next_task();

        if let Some(v) = self.role.consume_next_task() {
            for action in v.to_actions() {
                self.queue.push_back(action);
            }
        }
    }

    pub fn get_mut_action(&mut self) -> Option<&mut Action> {
        self.queue.front_mut()
    }

    pub fn get_action(&self) -> Option<&Action> {
        self.queue.front()
    }

    pub fn frame_update(&mut self) {
        // println!("{:?}", self.needs);
        self.needs.update();
    }

    fn deal_with_hungry(&mut self) {
        if self.inventory.get_qty(ItemEnum::MEAT) > 0 {
            self.queue
                .push_front(Action::CONSUME(ConsumeAction::new(ItemEnum::MEAT, 1)));
            return;
        }

        for action in BuyTask::new(ItemEnum::MEAT, 1, [100.0, 100.0, 0.0]).to_actions() {
            self.queue.push_back(action);
        }
    }
}
