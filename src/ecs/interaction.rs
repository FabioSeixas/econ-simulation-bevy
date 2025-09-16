use crate::ecs::trade::components::TradeNegotiation;

#[derive(Debug)]
pub enum AgentInteractionEvent {
    Trade(TradeNegotiation),
}
