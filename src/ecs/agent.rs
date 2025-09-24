use crate::core::{inventory::*, item::ItemEnum, needs::*};
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Agent {
    pub needs: Needs,
    pub inventory: Inventory,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            needs: Needs::new(),
            inventory: Inventory::new(),
        }
    }

    pub fn new_seller_of(item: ItemEnum) -> Self {
        let mut inv = Inventory::new();

        inv.add(item, 5000);

        Self {
            needs: Needs::new(),
            inventory: inv,
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
