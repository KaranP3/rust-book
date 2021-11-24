use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("the secret number is {}", secret_number);

    loop {
        println!("Please input your guess...");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "too small".red()),
            Ordering::Equal => {
                println!("{}", "you win".green());
                break;
            }
            Ordering::Greater => println!("{}", "too large".red()),
        }

        println!("{}", guess);
    }
}
