use colored::*;
use rand::prelude::*;

use std::cmp::Ordering;
use std::io::stdin;
use std::process;

fn main() {
    let welcome = String::from("Welcome to the guessing number game!");
    let winner = String::from("You win");
    let loser = String::from("Game over");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    let mut life = 4;

    println!("{}", welcome.cyan());

    while life > 0 {
        let mut guess = String::new();
        println!("[Life: {}] Please enter your guess:", life);

        stdin()
            .read_line(&mut guess)
            .expect("Failed reading your guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("You must enter a valid number");
                life -= 1;
                continue;
            }
        };

        if guess <= 0 || guess > 100 {
            println!("Your number needs to be between 1 and 100");
            life -= 1;
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Greater => {
                println!("Your guess is higher");
                life -= 1;
            }
            Ordering::Equal => {
                println!("{}", winner.green());
                process::exit(0);
            }
            Ordering::Less => {
                println!("Your guess is lower");
                life -= 1;
            }
        }
    }

    println!("{}", loser.red());
    println!("The number was {}", secret_number);
}
