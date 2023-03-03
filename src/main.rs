use sha_generator::take_first_n_chars;

mod group_utils;
mod sha_generator;
mod subset_generator;

fn main() {
    let result = take_first_n_chars(sha_generator::sha_generate_address(), 20);
    println!("{}", result)
}
