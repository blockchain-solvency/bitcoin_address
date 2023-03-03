use crate::group_utils::BaseGroup;
use rand::{distributions::Uniform, seq::SliceRandom, Rng}; // 0.6.5

pub fn generate_address() {
    let mut rng = rand::thread_rng();
    //TODO: We should use a 256 bit prime number for the upper bound
    let range = Uniform::new(1, 5000);
    let secret_set: Vec<u64> = (0..1024).map(|_| rng.sample(&range)).collect();
    let G = BaseGroup::new();
    let Y_1024: Vec<(primitive_types::U512, primitive_types::U512)> =
        secret_set.iter().map(|n| G.exponentiate(*n)).collect();
    println!("{:?} ", Y_1024);
    let Y_512 = Y_1024.choose_multiple(&mut rand::thread_rng(), 512);
}
