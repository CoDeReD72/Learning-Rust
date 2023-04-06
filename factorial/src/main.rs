fn main() {

    let number = 5;

    println!("The factorial of {} is {}", number, factorial(number));
}

fn factorial(number : i32) -> i32{
    let mut result = 1;
    for i in 1..=number {
        result *= i;
    }
    result
}
