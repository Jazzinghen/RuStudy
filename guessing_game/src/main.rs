use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        let parsed_guess = match guess.trim().parse::<u32>() {
            Ok(za_guess) => za_guess,
            Err(_) => {
                println!("Please enter a number!");
                continue
            }
        };

        println!("You guessed: {}", parsed_guess);

        match parsed_guess.cmp(&secret_number) {
            Ordering::Less => println!("Sorry, the value is larger"),
            Ordering::Greater => println!("Sorry, the value is smaller"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}