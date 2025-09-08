use bevy::prelude::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum NeedType {
    EAT,
    DRINK,
    SLEEP,
}

impl NeedType {
    fn as_index(&self) -> usize {
        match self {
            NeedType::EAT => 0,
            NeedType::DRINK => 1,
            NeedType::SLEEP => 2,
        }
    }
}

pub const NEED_THRESHOLD: usize = 500;

const LOCATIONS: [Vec3; 3] = [
    Vec3::new(300., 300., 0.), // EAT
    Vec3::new(400., 300., 0.), // DRINK
    Vec3::new(200., 200., 0.), // SLEEP
];

pub fn get_location(need: NeedType) -> Vec3 {
    LOCATIONS[need.as_index()]
}
