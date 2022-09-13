use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Try to guess the number (between 1 and 100)!");

    let rando_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Shadowing and removing the \n from user hitting enter
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // just ask for a new input
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&rando_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}