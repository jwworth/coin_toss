extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Coin Toss Game!");
    let number = rand::thread_rng().gen_range(0,2);

    loop {
        println!("Heads or tails?");
        println!("Please input your choice (0 or 1):");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .ok()
            .expect("Please type a number!");

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
