use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("Welcome to Rust Guessing Game");

    let mut guess = String::new();
    let mut rng = rand::thread_rng();
    let mut act = false;

    while !act {
        println!("Please enter number to guess:");

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let trimmed_guess = guess.trim();
        if trimmed_guess.is_empty() {
            println!("Please enter a valid number.");
            continue;
        }

        let parsed_guess = match trimmed_guess.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        let random_number = rng.gen_range(1..=10);

        if parsed_guess == random_number {
            println!("You guessed it right!");
            act = true;
        } else {
            println!("You guessed it wrong! The number was {}\n", random_number);
        }

        guess.clear();
    }
}
