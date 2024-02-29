use add_one;
use rand::prelude::*;

fn main() {
    // add_one::add(2,3);
    println!("Addition is: {}", add_one::add(2,3));
    let mut thread_rng = rand::thread_rng();
    let number = thread_rng.gen_range(0..100);
    println!("number generated is: {}", number);
}
