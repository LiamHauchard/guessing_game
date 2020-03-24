use colored::*;
use rand::prelude::*;

use std::cmp::Ordering;
use std::io::stdin;
use std::process;

fn main() {
    let welcome = String::from("Welcome to the guessing number game!");
    let warning = String::from("[warn] You must enter a valid number between 1 and 100");
    let winner = String::from("You win");
    let loser = String::from("Game over");

    let secret_number: u8 = rand::thread_rng().gen_range(1, 101);
    let mut life = 4;

    println!("{}", welcome.bright_purple());

    while life > 0 {
        let mut guess = String::new();
        println!("[Life: {}] Please enter your guess:", life);

        stdin()
            .read_line(&mut guess)
            .expect("Failed reading your guess");

        let guess: u8 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("{}", warning.bright_yellow());
                life -= 1;
                continue;
            }
        };

        if guess <= 0 || guess > 100 {
            println!("{}", warning.bright_yellow());
            life -= 1;
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Greater => {
                println!("Your guess is higher");
                life -= 1;
            }
            Ordering::Equal => {
                println!("{}", winner.bright_green());
                process::exit(0);
            }
            Ordering::Less => {
                println!("Your guess is lower");
                life -= 1;
            }
        }
    }

    let info = format!("[info] The number was {}", secret_number);
    println!("{}", info.bright_blue());

    println!("{}", loser.bright_red());
}
