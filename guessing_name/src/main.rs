use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        // This is _very_ interesting. It seems like you can willingly shadown a variable
        // with a new one by using the same name, which is great in the case const
        // conversion and such.

        // We can provide a type by usign ": type" which will also inform the parse call of
        // which type we want

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // Matching is a great function. I love it.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
