use crate::core::{
    action::*,
    inventory::*,
    item::ItemEnum,
    needs::*,
    role::{get_random_role, get_seller_role, Role},
    task::*,
};
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
pub struct Agent {
    pub needs: Needs,
    pub inventory: Inventory,
    action_queue: VecDeque<Action>,
    pub role: Box<dyn Role + Send + Sync>,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            needs: Needs::new(),
            inventory: Inventory::new(),
            action_queue: VecDeque::new(),
            role: get_random_role(),
        }
    }

    pub fn new_seller() -> Self {
        let mut inv = Inventory::new();
        inv.add(ItemEnum::MEAT, 5000);

        Self {
            needs: Needs::new(),
            inventory: inv,
            action_queue: VecDeque::new(),
            role: get_seller_role(),
        }
    }

    pub fn pop_current_action(&mut self) {
        let completed_action = self.action_queue.pop_front();
        // println!("action completed. {:?}", completed_action);
        // println!("Current Inventory: {:?}", self.inventory);
    }

    pub fn satisfy_hungry(&mut self) {
        self.needs.satisfy_hunger();
    }

    pub fn new_action(&mut self) {
        // println!("new action");
        if self.needs.is_hungry() {
            // println!("deal_with_hungry");
            return self.deal_with_hungry();
        }

        self.role.calculate_next_task();

        if let Some(v) = self.role.consume_next_task() {
            for action in v.to_actions() {
                // println!("adding action to queue {:?}", action);
                self.action_queue.push_back(action);
            }
        }
    }

    pub fn get_mut_action(&mut self) -> Option<&mut Action> {
        self.action_queue.front_mut()
    }

    pub fn get_action(&self) -> Option<&Action> {
        self.action_queue.front()
    }

    pub fn frame_update(&mut self) {
        // println!("{:?}", self.needs);
        self.needs.update();
    }

    fn deal_with_hungry(&mut self) {
        if self.inventory.get_qty(ItemEnum::MEAT) > 0 {
            self.action_queue
                .push_front(Action::CONSUME(ConsumeAction::new(ItemEnum::MEAT, 1)));
            self.action_queue
                .push_front(Action::Walk(Walk::new_random()));

            return;
        }

        for action in BuyTask::new(ItemEnum::MEAT, 1, [100., 100., 0.]).to_actions() {
            self.action_queue.push_back(action);
        }
    }
}
