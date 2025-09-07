use std::collections::HashMap;

use crate::item::ItemEnum;

#[derive(Debug)]
pub struct Inventory {
    items: HashMap<ItemEnum, u8>, // item id, item quantity
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: ItemEnum, qty: u8) {
        match self.items.get(&id) {
            None => {
                self.items.insert(id, qty);
            }
            Some(current_qty) => {
                self.items.insert(id, *current_qty + qty);
            }
        }
    }

    pub fn get_qty(&self, id: ItemEnum) -> u8 {
        match self.items.get(&id) {
            None => 0,
            Some(current_qty) => *current_qty,
        }
    }

    pub fn remove(&mut self, id: ItemEnum, qty: u8) -> u8 {
        match self.items.get(&id) {
            None => 0,
            Some(current_qty) => {
                if *current_qty < qty {
                    panic!("Not enought on inventory")
                }

                let rest = *current_qty - qty;
                self.items.insert(id, rest);
                rest
            }
        }
    }
}
