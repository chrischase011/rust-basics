mod guess;

use guess::guess;
use std::io::{self, Write};

fn main() {
    println!("Welcome to Rust Basics\n\nChoose an option...");

    print!("1. Guessing Game\n2. Exit\n> ");
    let mut _ch = String::new();
    io::stdout().flush().unwrap();

    
    io::stdin()
        .read_line(&mut _ch)
        .expect("Failed to read line");

    let ch: i32 = match _ch.trim().parse() {
        Ok(ch) => ch,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    match ch {
        1 => guess(),
        _ => println!("Hi there!"),
    }
}
