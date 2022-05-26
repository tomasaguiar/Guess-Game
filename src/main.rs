use std::io;
use rand::Rng;
use colored::*;
use std::cmp::Ordering;

fn main() {
    println!("The Guessing Game");
    println!("=================");

    let secret_key = rand::thread_rng().gen_range(1, 101);

    // println!("your secret number is {}", secret_key);
    println!("Guess a number between 1 and 100");

    loop {
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            // if input ins't a number, return an error.
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_key) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }}
}
