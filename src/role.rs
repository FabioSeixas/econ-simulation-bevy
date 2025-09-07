// use rand::prelude::*;

pub trait Role: Sync + Send + std::fmt::Debug {
    fn get_name(&self) -> &str;
}

#[derive(Debug)]
pub struct Seller;

#[derive(Debug)]
pub struct NoRole;

impl Role for Seller {
    fn get_name(&self) -> &str {
        "Seller"
    }
}

impl Role for NoRole {
    fn get_name(&self) -> &str {
        "No Role"
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
