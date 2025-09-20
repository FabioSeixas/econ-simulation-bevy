use crate::ecs::trade::components::TradeNegotiation;

#[derive(Debug)]
pub struct AgentInteractionEvent {
    pub id: usize,
    pub kind: AgentInteractionKind
}

#[derive(Debug)]
pub enum AgentInteractionKind {
    Trade(TradeNegotiation),
}
