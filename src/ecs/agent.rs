use crate::core::{action::*, inventory::*, item::ItemEnum, location::*, needs::*, task::Task};
use bevy::prelude::*;
use rand::Rng;
use std::collections::VecDeque;

#[derive(Component)]
pub struct Agent {
    pub entity_id: Entity,
    pub needs: Needs,
    pub inventory: Inventory,
    queue: VecDeque<Action>,
    // pub role: Box<dyn Role + Send + Sync>,
}

impl Agent {
    pub fn new(entity_id: Entity) -> Self {
        Self {
            needs: Needs::new(),
            inventory: Inventory::new(),
            queue: VecDeque::new(),
            entity_id,
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
            if self.inventory.get_qty(ItemEnum::MEAT) > 0 {
                self.queue
                    .push_front(Action::CONSUME(ConsumeAction::new(ItemEnum::MEAT, 1)));
                return;
            }

            for action in Task::new(1, "Get Food", [100.0, 100.0, 0.0]).to_actions() {
                self.queue.push_back(action);
            }
            // self.needs.eat_queued();
            return;
        }

        let mut rnd = rand::thread_rng();
        let max = 500.;

        let destination: Location = [rnd.gen_range(-max..max), rnd.gen_range(-max..max), 0.];

        self.queue.push_front(Action::Walk(Walk::new(destination)));
    }

    pub fn get_mut_action(&mut self) -> Option<&mut Action> {
        self.queue.front_mut()
    }

    pub fn get_action(&self) -> Option<&Action> {
        self.queue.front()
    }

    pub fn frame_update(&mut self) {
        println!("{:?}", self.needs);
        self.needs.update();
    }
}
