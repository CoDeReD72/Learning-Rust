fn main() {
    
    let mut x = 5; // mut used to make variable mutable - able to be changed after initialisation
    println!("The value of x is: {x}");
    x = 6; // Returns error if not mutable
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Constant - cannot be changed after initialisation
    // Constants can include basic evaluation
    // Naming = ALL_CAPS
    // Type must be declared

    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    /*
    CONCEPT OF SHADOWING

    If a variable is declared with the same name as a previous variable, the previous variable is shadowed
    
    The compiler only sees the current version of the variable, not the previous versions

    This remains until shadowed again or goes out of scope

    Difference between mut and let is that mut can change the type of the variable, whereas let cannot - essentially creates a new variable

    In shadowing the type can be changed when initialising the variable but after that the value cannot unless mutable

    */

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    

}
