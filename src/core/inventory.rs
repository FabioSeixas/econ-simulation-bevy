use std::collections::HashMap;

use crate::core::item::*;

#[derive(Debug)]
pub struct Inventory {
    items: HashMap<ItemEnum, usize>, // item id, item quantity
}

impl Inventory {
    pub fn new_seller() -> Self {
        let mut items = HashMap::new();

        items.insert(ItemEnum::MEAT, 100);
        items.insert(ItemEnum::WATER, 100);
        items.insert(ItemEnum::MONEY, 50);

        Self { items }
    }

    pub fn new() -> Self {
        let mut items = HashMap::new();

        items.insert(ItemEnum::MONEY, 20);

        Self { items }
    }

    pub fn add(&mut self, id: ItemEnum, qty: usize) {
        match self.items.get(&id) {
            None => {
                self.items.insert(id, qty);
            }
            Some(current_qty) => {
                self.items.insert(id, *current_qty + qty);
            }
        }
    }

    pub fn get_qty(&self, id: ItemEnum) -> usize {
        match self.items.get(&id) {
            None => 0,
            Some(current_qty) => *current_qty,
        }
    }

    pub fn list(&self) -> Vec<(ItemEnum, usize)> {
        self.items
            .iter()
            .map(|(item, qty)| (item.clone(), qty.clone()))
            .collect()
    }

    pub fn remove(&mut self, id: ItemEnum, qty: usize) -> usize {
        match self.items.get(&id) {
            None => 0,
            Some(current_qty) => {
                if *current_qty < qty {
                    panic!("Not enought {:?} on inventory", id)
                }

                let rest = *current_qty - qty;
                self.items.insert(id, rest);
                print!("removed {} of {:?} from inventory", qty, id);
                rest
            }
        }
    }
}
