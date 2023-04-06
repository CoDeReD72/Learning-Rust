fn main() {

    // Integer Types
    //u8, i8 - unisgned/signed 8-bit integer
    //u16, i16 - unisgned/signed 16-bit integer
    //u32, i32 - unisgned/signed 32-bit integer
    //u64, i64 - unisgned/signed 64-bit integer
    //u128, i128 - unisgned/signed 128-bit integer
    
    // Panics if the number is too large for the data type
    // If used --release: WRAPS AROUND IN THE CASE OF OVERFLOW so in 8-bit integer, 255 + 1 = 0

    // Floating Point Types
    // f32 - 32-bit floating point number
    // f64 - 64-bit floating point number - DEFAULT

    // Numeric Operations
    // +, -, *, /, %
    // Rust does not have ++ or -- operators

    let sum = 5 + 10;
    println!("5 + 10 = {}", sum);

    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", difference);

    let product = 4 * 30;
    println!("4 * 30 = {}", product);

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1 (Integer division truncates the decimal)
    println!("56.7 / 32.2 = {}", quotient);
    println!("-5 / 3 = {} due to Truncation", truncated);

    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder);

    // Boolean Type
    let _t: bool = true;
    let _f: bool = false;

    // Character Type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Always use ' ' for characters, " " for strings or string literals

    // Compound Types
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1); // Tuple with 3 values of different types
    let (_x, y, _z) = tup; // Destructuring - binds each value to a variable
    println!("The value of y is: {}", y); // Prints 6.4

    let _five_hundred = tup.0; // Accessing tuple values with a period and index
    let _six_point_four = tup.1;
    let _one = tup.2;

    // Array - Fixed number of elements
    // Data is allocated to the stack - not the heap
    // All elements must be of the same type
    // Vectors are a similar type to array, but is allocated to the heap and can grow or shrink in size

    let a = [1, 2, 3, 4, 5]; // Array with 5 values of the same type
    let _first = a[0]; // Accessing array values with a square bracket and index

    let _months : [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // Array index out of bounds error
    
    // let _index_out_of_bounds = a[10]; // Panics with a message & Returns a Result type with an Err variant
    
    // If through user input a RUNTIME ERROR will occur - No way to predict what the user will input


}
