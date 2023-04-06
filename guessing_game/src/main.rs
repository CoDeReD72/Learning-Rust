use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // rand is a crate (think import), thread_rng is a function to generate random numbers, gen_range is a function to generate a random number between 1 and 100

    println!("The secret number is: {}", secret_number); // Remove later

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // String is a type, ::new is an associated function that creates a new instance of a String

        io::stdin()
            .read_line(&mut guess) // read_line similar to Java Scanner, but &type name needs to be used to bind to a string
            // Returns a Result type, which is an enum with Ok and Err variants
            .expect("Failed to read line"); // expect is similar to Java try/catch - if read_line fails, expect will print the string

        // A warning will be given if .expect is not used, as it is a Result type and not a String type

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        }; // trim is a method to remove whitespace, parse is a method to convert a string to a number, expect is a method to print a string if the parse fails

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // match is similar to switch statement in Java - guess.cmp is a method that compares two values
            // & is used to bind to a reference of the value - this is because cmp takes a reference
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
