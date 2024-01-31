/// Prints "Testing Mutability" and demonstrates mutable and immutable variables.
/// Also prints the value of `x` and `y` variables. Defines a constant
/// `THREE_HOURS_IN_SECONDS` and prints its value.
fn main() {
    println!("\nTesting Mutability");

    // Use let mut to define mutable variable
    let mut x = 5;

    // Use let to define immutable variable
    let y = 5;

    println!("The value of x is: {x} {y}");

    // Reassign value of mutable variable
    x = 10;

    // Define constant with const
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("here is a constant: {THREE_HOURS_IN_SECONDS}");
}
