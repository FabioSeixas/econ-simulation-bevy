#[derive(Debug)]
pub struct Needs {
    pub hunger: usize,
    pub thirst: usize,
    pub sleep: usize,
    // eat_queued: bool,
}

impl Needs {
    pub fn new() -> Self {
        Self {
            hunger: 0,
            thirst: 0,
            sleep: 0,
            // eat_queued: false
        }
    }

    pub fn update(&mut self) {
        self.hunger += 1;
        self.thirst += 1;
        self.sleep += 1;
    }

    pub fn is_hungry(&self) -> bool {
        self.hunger > 500 
    }

    pub fn satisfy_hunger(&mut self) {
        self.hunger = 0;
    }

    // pub fn eat_queued(&mut self) {
    //     self.eat_queued = true
    // }
}
