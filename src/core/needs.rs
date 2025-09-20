#[derive(Debug)]
pub struct Needs {
    pub hunger: usize,
    pub thirst: usize,
    pub sleep: usize,
}

impl Needs {
    pub fn new() -> Self {
        Self {
            hunger: 0,
            thirst: 0,
            sleep: 0,
        }
    }

    pub fn update(&mut self) {
        self.hunger += 1;
        self.thirst += 1;
        self.sleep += 1;
    }

    pub fn is_hungry(&self) -> bool {
        self.hunger > 1000 
    }

    pub fn is_thirsty(&self) -> bool {
        self.thirst > 1000 
    }

    pub fn satisfy_hunger(&mut self) {
        self.hunger = 0;
    }

    pub fn satisfy_thirsty(&mut self) {
        self.thirst = 0;
    }
}
