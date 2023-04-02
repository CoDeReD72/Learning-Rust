use std::io;

fn main(){
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // mut means mutable - Variable cannot be changed otherwise. String initialised to empty string.

    io::stdin().read_line(&mut guess) // read_line similar to Java Scanner, but &type name needs to be used to bind to a string
    // Returns a Result type, which is an enum with Ok and Err variants
        .expect("Failed to read line"); // expect is similar to Java try/catch - if read_line fails, expect will print the string
        
        // A warning will be given if .expect is not used, as it is a Result type and not a String type

    println!("You guessed: {}", guess);
}