use crate::{core::item::ItemEnum, Agent, BuyAction, ConsumeAction};

pub fn buy_action_callback(agent: &mut Agent, buy_action: &BuyAction) {
    if buy_action.price_paid.is_none() {
        panic!("callback called without price paid")
    }

    agent.inventory.add(buy_action.item.clone(), buy_action.qty);
    agent
        .inventory
        .remove(ItemEnum::MONEY, buy_action.price_paid.unwrap());
}

pub fn consume_action_callback(agent: &mut Agent, consume_action: &ConsumeAction) {
    let item = consume_action.item.clone();
    if item.is_food() {
        agent.satisfy_hungry();
    }
    agent.inventory.remove(item, consume_action.qty);
}
