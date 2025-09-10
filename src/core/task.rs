use crate::core::{action::*, item::*, location::*};

pub trait Task {
    fn to_actions(&self) -> Vec<Action>;
}

#[derive(Debug)]
pub struct WalkTask {
    pub destination: Location,
}

impl WalkTask {
    pub fn new(destination: Location) -> Self {
        Self { destination }
    }
}

impl Task for WalkTask {
    fn to_actions(&self) -> Vec<Action> {
        vec![Action::Walk(Walk::new(self.destination))]
    }
}

#[derive(Debug)]
pub struct BuyTask {
    pub qty: usize,
    pub item: ItemEnum,
    pub location: Location,
}

impl BuyTask {
    pub fn new(item: ItemEnum, qty: usize, location: Location) -> Self {
        Self {
            item,
            qty,
            location,
        }
    }
}

impl Task for BuyTask {
    fn to_actions(&self) -> Vec<Action> {
        vec![
            Action::Walk(Walk::new(self.location)),
            Action::BUY(BuyAction::new(self.item.clone(), self.qty)),
        ]
    }
}
