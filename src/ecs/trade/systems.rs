use bevy::prelude::*;

use crate::{
    core::item::ItemEnum,
    ecs::{
        agent::Agent,
        components::Interacting,
        trade::{
            components::{BuyTask, Buying, Selling, TradeInteraction, TradeNegotiation, TradeRole},
            events::{OfferAgreed, OfferMade, TradeFinalized},
        },
    },
    Idle,
};

pub fn seller_makes_offer_system(
    mut seller_query: Query<
        (&Agent, &mut TradeNegotiation),
        (With<Selling>, Added<TradeNegotiation>, With<Interacting>),
    >,
    mut offer_made_writer: EventWriter<OfferMade>,
    mut trade_finalized_writer: EventWriter<TradeFinalized>,
) {
    for (agent, mut trade) in &mut seller_query {
        let seller_amount = agent.inventory.get_qty(trade.item);

        if seller_amount == 0 {
            trade_finalized_writer.send(TradeFinalized {
                target: trade.partner,
                success: false,
            });
            continue;
        }

        if seller_amount < trade.quantity {
            trade.quantity = seller_amount;
        }
        let price = trade.quantity * 3;
        trade.price = Some(price);

        offer_made_writer.send(OfferMade {
            target: trade.partner,
            quantity: trade.quantity,
            price,
        });
    }
}

pub fn buyer_evaluates_offer_system(
    mut buyer_query: Query<
        (Entity, &Agent, &TradeNegotiation),
        (With<Buying>, With<TradeNegotiation>, With<Interacting>),
    >,
    mut offer_agreed_writer: EventWriter<OfferAgreed>,
    mut offer_made_reader: EventReader<OfferMade>,
    mut trade_finalized_writer: EventWriter<TradeFinalized>,
) {
    for event in offer_made_reader.read() {
        if let Ok((entity, agent, trade)) = buyer_query.get_mut(event.target) {
            if agent.inventory.get_qty(ItemEnum::MONEY) >= event.price {
                // trade.quantity = event.quantity;
                // trade.price = Some(event.price);

                println!("buyer_evaluates_offer_system: {:?}", trade);
                offer_agreed_writer.send(OfferAgreed {
                    target: trade.partner,
                    price: event.price,
                    quantity: event.quantity,
                });

                offer_agreed_writer.send(OfferAgreed {
                    target: entity,
                    price: event.price,
                    quantity: event.quantity,
                });
            } else {
                // TODO: improve (decrease qty, make a new offer...)
                trade_finalized_writer.send(TradeFinalized {
                    target: trade.partner,
                    success: false,
                });

                trade_finalized_writer.send(TradeFinalized {
                    target: entity,
                    success: false,
                });
            }
        } else {
            println!("No target agent found for event: {:?}", event);
        }
    }
}

pub fn handle_offer_agreed_system(
    mut target_query: Query<(&mut Agent, &TradeNegotiation), With<Interacting>>,
    mut offer_agreed_reader: EventReader<OfferAgreed>,
    mut trade_finalized_writer: EventWriter<TradeFinalized>,
) {
    for event in offer_agreed_reader.read() {
        if let Ok((mut agent, trade)) = target_query.get_mut(event.target) {
            let price = event.price;
            let quantity = event.quantity;
            if trade.role == TradeRole::Buyer {
                agent.inventory.remove(ItemEnum::MONEY, price * quantity);
                agent.inventory.add(trade.item, quantity);
            } else {
                agent.inventory.add(ItemEnum::MONEY, price * quantity);
                agent.inventory.remove(trade.item, quantity);
            }

            trade_finalized_writer.send(TradeFinalized {
                target: event.target,
                success: true,
            });
        } else {
            println!("No target agent found for event: {:?}", event);
        }
    }
}

pub fn handle_trade_finalized(
    mut target_query: Query<(&TradeNegotiation, Option<&mut BuyTask>), With<Interacting>>,
    mut trade_finalized_reader: EventReader<TradeFinalized>,
    mut commands: Commands,
) {
    for event in trade_finalized_reader.read() {
        if let Ok((trade, buy_task)) = target_query.get_mut(event.target) {
            if trade.role == TradeRole::Buyer {
                if event.success {
                    commands
                        .entity(event.target)
                        .insert(Idle)
                        .remove::<(TradeInteraction, Buying, BuyTask)>();
                } else {
                    buy_task
                        .expect("handle_trade_finalized -> buy task must be Some here!")
                        .add_tried(trade.partner);
                    commands
                        .entity(event.target)
                        .remove::<(TradeInteraction, Buying)>();
                }
            } else {
                commands.entity(event.target).remove::<TradeInteraction>();
            }
        } else {
            println!("No target agent found for event: {:?}", event);
        }
    }
}
