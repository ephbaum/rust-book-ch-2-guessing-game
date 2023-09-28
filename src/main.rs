use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number! (Ctrl-C to quit)");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut num_guesses = 0;

    loop {
        println!("Please input your guess between 1 and 100");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        num_guesses += 1;

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number between 1 and 100.");
                continue;
            }
        };

        if guess > 100 {
            println!("Invalid guess, the number must be between 1 and 100.");
            println!("Try again!");
            continue;
        }

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You took {num_guesses} guesses");
                break;
            }
        }
    }
}
