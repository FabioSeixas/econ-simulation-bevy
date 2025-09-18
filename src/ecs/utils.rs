use bevy::math::Vec3;
use rand::Rng;

const MAX: f32 = 500.;

pub fn get_random_vec3() -> Vec3 {
    let mut rnd = rand::thread_rng();
    Vec3::new(rnd.gen_range(-MAX..MAX), rnd.gen_range(-MAX..MAX), 0.)
}
