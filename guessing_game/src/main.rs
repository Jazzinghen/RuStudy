use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1, 100);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line!");
    let parsed_guess = guess.trim().parse::<u32>().expect("Please provide an integer next time.");

    println!("You guessed: {}", parsed_guess);

    match parsed_guess.cmp(&secret_number) {
        Ordering::Less => println!("Sorry, the value is larger: {}", secret_number),
        Ordering::Greater => println!("Sorry, the value is smaller: {}", secret_number),
        Ordering::Equal => println!("You win!")
    }
}