#[derive(Debug, Clone, PartialEq)]
pub enum ActionType {
    WALK,
    EAT,
    DRINK,
    WORK,
    BUY,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ActionState {
    CREATED,     // Action is initialized
    WAITING,     // Waiting for conditions (e.g., resources)
    IN_PROGRESS, // Currently being executed
    COMPLETED,   // Action is finished
}

#[derive(Debug)]
pub struct Action {
    pub action_type: ActionType,
    pub state: ActionState,
    pub duration: u32, // Duration for actions that take time
}

impl Action {
    pub fn new(action_type: ActionType, duration: u32) -> Self {
        Self {
            action_type,
            state: ActionState::CREATED,
            duration,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.state == ActionState::COMPLETED
    }
}

trait ActionTrait {
    fn is_complete(&self) -> bool;
    fn current_state(&self) -> ActionState;
}

struct WalkAction;

impl ActionTrait for WalkAction {
    fn current_state(&self) -> ActionState {
        ActionState::CREATED
    }

    fn is_complete(&self) -> bool {
        true
    }
}

fn testing() {
    let mut x = WalkAction {};
}
