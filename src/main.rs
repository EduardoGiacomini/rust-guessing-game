use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let secret_values_range = 1..=100;
    let secret_number = rand::thread_rng().gen_range(secret_values_range);

    println!("Hello, welcome to the Guessing Number Game.");
    println!("I will think about one number from 1 to 100 and you have to guess it. Let's start.");

    loop {
        let mut guess = String::new();

        println!("Please, input your guess:");

        io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read the input guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
        }
    }
}
