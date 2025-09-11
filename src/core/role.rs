use super::location::*;
use super::task::*;
use rand::Rng;

pub trait Role: Sync + Send + std::fmt::Debug {
    fn get_name(&self) -> &str;
    fn get_next_task(&self) -> Option<&dyn Task>;
    fn consume_next_task(&mut self) -> Option<Box<dyn Task>>;
    fn calculate_next_task(&mut self);
}

#[derive(Debug)]
pub struct NoRole {
    current_task: Option<Box<dyn Task>>,
}

impl NoRole {
    pub fn new() -> Self {
        Self { current_task: None }
    }
}

impl Role for NoRole {
    fn get_name(&self) -> &str {
        "No Role"
    }

    fn calculate_next_task(&mut self) {
        if self.current_task.is_none() {
            let mut rnd = rand::thread_rng();
            let max = 500.;
            let task = Box::new(WalkTask::new([
                rnd.gen_range(-max..max),
                rnd.gen_range(-max..max),
                0.,
            ]));
            self.current_task = Some(task);
        }
    }

    fn get_next_task(&self) -> Option<&dyn Task> {
        self.current_task.as_deref()
    }

    fn consume_next_task(&mut self) -> Option<Box<dyn Task>> {
        self.current_task.take()
    }
}

#[derive(Debug)]
pub struct Seller {
    current_task: Option<Box<dyn Task>>,
    location: Location,
}

impl Seller {
    pub fn new(location: Location) -> Self {
        Self {
            current_task: None,
            location,
        }
    }
}

impl Role for Seller {
    fn get_name(&self) -> &str {
        "Seller"
    }

    fn calculate_next_task(&mut self) {
        if self.current_task.is_none() {
            let task = Box::new(SellTask::new(self.location));
            self.current_task = Some(task);
        }
    }

    fn get_next_task(&self) -> Option<&dyn Task> {
        self.current_task.as_deref()
    }

    fn consume_next_task(&mut self) -> Option<Box<dyn Task>> {
        self.current_task.take()
    }
}

pub fn get_seller_role() -> Box<dyn Role + Send + Sync> {
    // let mut rnd = rand::thread_rng();
    // let max = 500.;
    // let location = [rnd.gen_range(-max..max), rnd.gen_range(-max..max), 0.];
    Box::new(Seller::new([100.0, 100.0, 0.0]))
}

// not random for now
pub fn get_random_role() -> Box<dyn Role + Send + Sync> {
    Box::new(NoRole::new())
}
