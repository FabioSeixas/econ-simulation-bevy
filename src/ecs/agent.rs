use crate::core::{action::*, inventory::*, item::ItemEnum, needs::*};
use bevy::prelude::*;
use rand::Rng;
use std::collections::VecDeque;

#[derive(Component, Debug)]
pub struct Agent {
    pub needs: Needs,
    pub inventory: Inventory,
    action_queue: VecDeque<Action>,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            needs: Needs::new(),
            inventory: Inventory::new(),
            action_queue: VecDeque::new(),
        }
    }

    pub fn new_seller() -> Self {
        let mut rng = rand::thread_rng();
        let mut inv = Inventory::new();

        if rng.gen_bool(0.5) {
            inv.add(ItemEnum::MEAT, 5000);
        } else {
            inv.add(ItemEnum::WATER, 5000);
        }

        Self {
            needs: Needs::new(),
            inventory: inv,
            action_queue: VecDeque::new(),
        }
    }

    pub fn satisfy_hungry(&mut self) {
        self.needs.satisfy_hunger();
    }

    pub fn satisfy_thirsty(&mut self) {
        self.needs.satisfy_thirsty();
    }

    pub fn is_hungry(&self) -> bool {
        self.needs.is_hungry()
    }

    pub fn is_thirsty(&self) -> bool {
        self.needs.is_thirsty()
    }

    pub fn get_action(&self) -> Option<&Action> {
        self.action_queue.front()
    }

    pub fn frame_update(&mut self) {
        self.needs.update();
    }

    pub fn have_food(&self) -> bool {
        self.inventory.get_qty(ItemEnum::MEAT) > 0
    }

    pub fn have_drink(&self) -> bool {
        self.inventory.get_qty(ItemEnum::WATER) > 0
    }
}
