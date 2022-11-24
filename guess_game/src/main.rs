use std::{io, cmp::Ordering};
use rand::Rng;
use colored::*;
fn main() {
    println!("Guess a number!");

    println!("Please input a number: ");

    loop {
        let mut guess = String::new();

        let secret_number = rand::thread_rng().gen_range(1..=100);
    
        println!("secret number: {}", secret_number);
        io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
        
        println!("you guessed {}", guess);
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
