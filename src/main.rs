use std::io;

fn main() {
    println!("Heads or tails?");
    println!("Please input your choice (h or t):");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
