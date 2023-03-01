use rand::{distributions::Uniform, Rng}; // 0.6.5

fn main() {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(1, 5000);
    let secret_set: Vec<u64> = (0..1024).map(|_| rng.sample(&range)).collect();


}
