use super::task::Task;
use rand::Rng;

pub trait Role: Sync + Send + std::fmt::Debug {
    fn get_name(&self) -> &str;

    // fn tasks(&self) -> &Vec<Task>;
    // fn tasks_mut(&mut self) -> &mut Vec<Task>; // mutable access
    // fn get_next_task(&mut self, buf: &mut Task);
    fn get_next_task(&self) -> Task;
}

// #[derive(Debug)]
// pub struct Seller;
//
// impl Role for Seller {
//     fn get_name(&self) -> &str {
//         "Seller"
//     }
//
//     fn get_next_task(&mut self, buf: &mut Task) {
//         buf._id = 1;
//         buf._name = String::from("Sell");
//         buf._where = get_location(Nee::EAT);
//         buf._duration = 1000;
//         buf._action_type = ActionType::WORK;
//     }
// }

#[derive(Debug)]
pub struct NoRole;

impl Role for NoRole {
    fn get_name(&self) -> &str {
        "No Role"
    }

    fn get_next_task(&self) -> Task {
        let mut rnd = rand::thread_rng();
        let max = 500.;

        Task::new(
            1,
            "Just Walk",
            [rnd.gen_range(-max..max), rnd.gen_range(-max..max), 0.],
        )
    }
}

// pub fn get_seller_role() -> Box<dyn Role + Send + Sync> {
//     Box::new(Seller)
//     // let mut rng = thread_rng();
//     // match rng.gen_range(0..2) {
//     //     0 => Box::new(Seller),
//     //     _ => Box::new(NoRole),
//     // }
// }

// not random for now
pub fn get_random_role() -> Box<dyn Role + Send + Sync> {
    Box::new(NoRole)

    // let mut rng = thread_rng();
    // match rng.gen_range(0..2) {
    //     0 => Box::new(Seller),
    //     _ => Box::new(NoRole),
    // }
}
