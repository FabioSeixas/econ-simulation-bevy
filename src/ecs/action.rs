use crate::{core::item::ItemEnum, Agent, BuyAction, ConsumeAction};

pub fn buy_action_callback(agent: &mut Agent, buy_action: &BuyAction, failed: bool) {
    if failed {
        // TODO: some negative feedback for the agent
        // for now, the action will be poped out, and the agent will try again
    } else if buy_action.price_paid.is_none() {
        panic!("callback called without price paid")
    } else {
        agent.inventory.add(buy_action.item.clone(), buy_action.qty);
        agent
            .inventory
            .remove(ItemEnum::MONEY, buy_action.price_paid.unwrap());
    }

    agent.pop_current_action();
}

pub fn consume_action_callback(agent: &mut Agent, consume_action: &ConsumeAction, failed: bool) {
    if failed {
        // TODO: try consume later ??
    } else {
        let item = consume_action.item.clone();
        if item.is_food() {
            agent.satisfy_hungry();
        }
        agent.inventory.remove(item, consume_action.qty);
    }
    agent.pop_current_action();
}
