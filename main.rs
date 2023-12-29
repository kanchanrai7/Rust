//Guess the number Game in Rust

use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Enter your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");
        let guess: u32 = guess.trim().parse().expect("Please type a no");
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too large".red()),
            Ordering::Equal => {
                println!("{}", "You win!".yellow());
                break;
            }
        }
        println!("Your guess is: {}", guess);
    }
}
