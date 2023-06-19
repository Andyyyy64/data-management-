use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();

    println!("Rolling dice...");

    let die1: u32 = rng.gen_range(1..=6);
    println!("Die 1: {}", die1);

    let die2: u32 = rng.gen_range(1..=6);
    println!("Die 2: {}", die2);

    let total_value = die1 + die2;
    println!("Total value: {}", total_value);
}
