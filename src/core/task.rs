use crate::core::{action::*, item::*, location::*};

#[derive(Debug)]
pub struct Task {
    pub id: u8,
    pub name: String,
    pub target_location: Location,
}

impl Task {
    pub fn new(id: u8, name: &str, target_location: Location) -> Self {
        Self {
            id,
            name: name.to_string(),
            target_location,
        }
    }

    // Convert a task into associated actions
    pub fn to_actions(&self) -> Vec<Action> {
        vec![
            Action::Walk(Walk::new(self.target_location)),
            Action::BUY(BuyAction::new(ItemEnum::MEAT, 1)),
        ]
    }
}
