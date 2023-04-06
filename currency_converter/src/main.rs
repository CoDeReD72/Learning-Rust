use std::io;

fn main() {
    const GBP_TO_USD : f32 = 1.3;

    loop{

    println!("Enter a number to convert from GBP to USD.");
    let mut amount = String::new();

    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");

    let amount: f32 = match amount.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("{} GBP is {} USD", amount, amount * GBP_TO_USD);
    break;

    }

}
