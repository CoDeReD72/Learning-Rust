fn main() {
    println!("Hello, world!");

    another_function(5, 'm');
}

fn another_function(value : i32, unit : char) {
    println!("The measurement is {value}{unit}");
}

//snake_case is used for function names