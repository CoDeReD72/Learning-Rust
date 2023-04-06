use std::io;

fn main() {
    loop{
    println!("Enter a number to find the factorial of that number.");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number = match number.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,};

    println!("The factorial of {} is {}", number, factorial(number));
    break;
    }
}

fn factorial(number: i32) -> i32 {
    let mut result = 1;
    for i in 1..=number {
        result *= i;
    }
    result
}
