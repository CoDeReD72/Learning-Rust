fn main() {
    for i in 1..100{
        if i % 3 == 0 && i % 5 == 0{
            println!("{} - Fizz Buzz", i);
            continue;
        }
        if i % 3 == 0{
            println!("{} - Fizz", i);
            continue;
        }
        if i % 5 == 0{
            println!("{} - Buzz", i);
            continue;
        }
        else{
            println!("{}", i);
        }
    }
}