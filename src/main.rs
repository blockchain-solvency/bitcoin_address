mod group_utils;

use rand::{distributions::Uniform, Rng}; // 0.6.5
use group_utils::BaseGroup;
fn main() {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(1, 5000);
    let secret_set: Vec<u64> = (0..1024).map(|_| rng.sample(&range)).collect();
    let G = BaseGroup::new();
    let Y: Vec<(primitive_types::U512,primitive_types::U512)> = secret_set.iter().map(|n| G.exponentiate(*n)).collect();

}
