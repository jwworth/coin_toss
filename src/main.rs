extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("---------------");
    println!("Coin Toss Game!");

    loop {
        println!("Heads or tails?");
        println!("Please input your choice (0 or 1):");

        let number = rand::thread_rng().gen_range(0,2);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&number) {
              Ordering::Less    => println!("You lose!"),
              Ordering::Greater => println!("You lose!"),
              Ordering::Equal   => {
                  println!("You win!");
                  break;
              }
        }
    }
}
