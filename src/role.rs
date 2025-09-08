use bevy::prelude::Vec3;
use rand::prelude::*;

use crate::{locations::{get_location, NeedType}, task::Task, ActionType};

pub trait Role: Sync + Send + std::fmt::Debug {
    fn get_name(&self) -> &str;

    // fn tasks(&self) -> &Vec<Task>;
    // fn tasks_mut(&mut self) -> &mut Vec<Task>; // mutable access
    fn get_next_task(&mut self, buf: &mut Task);
}

#[derive(Debug)]
pub struct Seller;

impl Role for Seller {
    fn get_name(&self) -> &str {
        "Seller"
    }

    fn get_next_task(&mut self, buf: &mut Task) {
        buf._id = 1;
        buf._name = String::from("Sell");
        buf._where = get_location(NeedType::EAT);
        buf._duration = 1000;
        buf._action_type = ActionType::WORK;
    }
}

#[derive(Debug)]
pub struct NoRole;

impl Role for NoRole {
    fn get_name(&self) -> &str {
        "No Role"
    }

    fn get_next_task(&mut self, buf: &mut Task) {
        let mut rnd = rand::thread_rng();
        let max = 500.;

        buf._id = 1;
        buf._name = String::from("Just Walk");
        buf._where = Vec3 {
            x: rnd.gen_range(-max..max),
            y: rnd.gen_range(-max..max),
            z: 0.,
        };
        buf._duration = 0;
        buf._action_type = ActionType::WALK(buf._where);
    }
}

pub fn get_seller_role() -> Box<dyn Role + Send + Sync> {
    Box::new(Seller)
    // let mut rng = thread_rng();
    // match rng.gen_range(0..2) {
    //     0 => Box::new(Seller),
    //     _ => Box::new(NoRole),
    // }
}

// not random for now
pub fn get_random_role() -> Box<dyn Role + Send + Sync> {
    Box::new(NoRole)

    // let mut rng = thread_rng();
    // match rng.gen_range(0..2) {
    //     0 => Box::new(Seller),
    //     _ => Box::new(NoRole),
    // }
}
