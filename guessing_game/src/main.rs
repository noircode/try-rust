use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess your number!");

    loop {
        print!("Please input your guess: ");
        println!();

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("The input is invalid, please input number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}