extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Heads or tails?");
    println!("Please input your choice (0 or 1):");

    let number = rand::thread_rng().gen_range(0,1);
    println!("The coin toss result is: {}", number);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
