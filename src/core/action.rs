use super::location::Location;
use super::item::ItemEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionState {
    CREATED,     // Action is initialized
    WAITING,     // Waiting for conditions (e.g., resources)
    IN_PROGRESS, // Currently being executed
    COMPLETED,   // Action is finished
}

#[derive(Debug, Clone)]
pub enum Action {
    Walk(Walk),
    BUY(BuyAction),
    CONSUME(ConsumeAction),
    SELL(SellAction),
}

pub trait DestinationAction {
    fn get_destination(&self) -> Location;
}

pub trait DurationAction {
    fn get_resting_duration(&self) -> f32;
    fn progress(&mut self, time: f32);
}

pub trait StatefullAction {
    fn current_state(&self) -> ActionState;
    fn update_state(&mut self);
    fn complete(&mut self);
    // fn process(&mut self) -> ActionExecution;
}

// pub trait Action {
//     fn is_complete(&self) -> bool;
//     fn current_state(&self) -> ActionState;
//     fn update_state(&mut self, new_state: ActionState);
// }

#[derive(Debug, Clone)]
pub struct Walk {
    state: ActionState,
    destination: Location,
}

impl Walk {
    pub fn new(destination: Location) -> Self {
        Self {
            destination,
            state: ActionState::CREATED,
        }
    }
}

impl DestinationAction for Walk {
    fn get_destination(&self) -> Location {
        self.destination
    }
}

impl StatefullAction for Walk {
    fn current_state(&self) -> ActionState {
        self.state
    }

    fn complete(&mut self) {
        self.state = ActionState::COMPLETED;
    }

    fn update_state(&mut self) {
        match self.state {
            ActionState::CREATED => self.state = ActionState::IN_PROGRESS,
            ActionState::WAITING => self.state = ActionState::IN_PROGRESS,
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuyAction {
    // target_seller ???
    state: ActionState,
    pub item: ItemEnum,
    pub qty: usize,
    pub price_paid: Option<usize>,
}

impl BuyAction {
    pub fn new(item: ItemEnum, qty: usize) -> Self {
        Self {
            state: ActionState::CREATED,
            qty,
            item,
            price_paid: None,
        }
    }

    pub fn set_price_paid(&mut self, p: usize) {
        self.price_paid = Some(p);
    }
}

impl StatefullAction for BuyAction {
    fn current_state(&self) -> ActionState {
        self.state
    }

    fn complete(&mut self) {
        self.state = ActionState::COMPLETED;
    }

    fn update_state(&mut self) {
        match self.state {
            ActionState::CREATED => self.state = ActionState::WAITING,
            ActionState::WAITING => self.state = ActionState::IN_PROGRESS,
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConsumeAction {
    state: ActionState,
    duration: f32,
    resting_duration: f32,
    pub item: ItemEnum,
    pub qty: usize,
}

impl ConsumeAction {
    pub fn new(item: ItemEnum, qty: usize) -> Self {
        Self {
            state: ActionState::CREATED,
            qty,
            item,
            duration: 5. * (qty as f32),
            resting_duration: 5. * (qty as f32),
        }
    }
}

impl DurationAction for ConsumeAction {
    fn get_resting_duration(&self) -> f32 {
        self.resting_duration
    }
    fn progress(&mut self, time: f32) {
        self.resting_duration -= time;
    }
}

impl StatefullAction for ConsumeAction {
    fn current_state(&self) -> ActionState {
        self.state
    }

    fn complete(&mut self) {
        self.state = ActionState::COMPLETED;
    }

    fn update_state(&mut self) {
        match self.state {
            ActionState::CREATED => self.state = ActionState::IN_PROGRESS,
            ActionState::WAITING => self.state = ActionState::IN_PROGRESS,
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct SellAction {
    state: ActionState,
    duration: f32,
    resting_duration: f32,
}

impl SellAction {
    pub fn new() -> Self {
        Self {
            state: ActionState::CREATED,
            duration: 25., 
            resting_duration: 25.
        }
    }
}

impl DurationAction for SellAction {
    fn get_resting_duration(&self) -> f32 {
        self.resting_duration
    }
    fn progress(&mut self, time: f32) {
        self.resting_duration -= time;
    }
}

impl StatefullAction for SellAction {
    fn current_state(&self) -> ActionState {
        self.state
    }

    fn complete(&mut self) {
        self.state = ActionState::COMPLETED;
    }

    fn update_state(&mut self) {
        match self.state {
            ActionState::CREATED => self.state = ActionState::IN_PROGRESS,
            ActionState::WAITING => self.state = ActionState::IN_PROGRESS,
            ActionState::IN_PROGRESS => self.state = ActionState::WAITING, // negotiation happening
            _ => {}
        }
    }
}
