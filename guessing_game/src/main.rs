use rand::prelude::*; // For random number generation
use std::cmp::Ordering; // For Comparing numbers, based on the Order
use std::io; // For input/output

fn main() {
    println!("Hello, World!"); // Printing to terminal
    println!("This is a Guessing Game!");

    let secret_number = rand::rng().random_range(1..=100); // using rng():random number generator function, and calling random_range method to get number in a defined range

    loop {
        // loop
        println!("Please enter your guess: ");
        let mut guess = String::new(); // Creating New Empty String
        io::stdin() // using stdin() function for input
            .read_line(&mut guess) // read line and change value of guess using a mutable reference
            .expect("Failed to read line!"); // if Error, this will show on terminal as error message

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            // trimming spaces in the beginning and end of string and converting guess string into number using parse and putting that number guess into a match
            Ok(num) => num,     // if the value of guess.trim().parse() is a num, ok
            Err(_) => continue, // if not then continue loop from beginning
        };

        match guess.cmp(&secret_number) {
            // comparing guess with secret number and matching
            Ordering::Less => println!("Too Small!"), // if compare says less, then do this
            Ordering::Equal => {
                // if equal, execute this line
                println!("YOU WIN!");
                break; // and break the loop
            }
            Ordering::Greater => println!("Too Large!"), // if compare says Greater
        }
    }
}
