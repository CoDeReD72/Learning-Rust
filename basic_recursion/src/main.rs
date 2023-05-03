fn main() {
    println!("Factorial result = {}", factorial(4)); // 4! = 4 * 3 * 2 * 1 = 24
    println!("Fibonacci result = {}", fibonacci(5)); // 5th fibonacci number = 5
    println!("Threes result = {}", threes(7)); // 7 * 3 = 21
}

// Basic Recap of Recursion
// 1. Base Case
// 2. Recursive Case (calls itself with a different argument until it reaches the base case)


// Generate the factorial of a given number
fn factorial(n : i32) -> i32{
    if n == 1{
        return 1; // Base case
    }
    else {
        return n * factorial(n-1); // Recursive case
    }
}

fn fibonacci(n : i32) -> i32{
    if n == 0 {
        return 0; 
    }
    else if n == 1 {
        return 1;
    }
    else {
        return fibonacci(n-1) + fibonacci(n-2);
    }
}
 
// Generate threes of a given number
fn threes(n : i32) -> i32{
    if n == 0 {
        return 0;
    }
    else {
        return 3 + threes(n-1);
    }
}