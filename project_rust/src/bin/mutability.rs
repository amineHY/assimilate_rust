/// Prints "Testing Mutability" and demonstrates mutable and immutable variables.
/// Initializes an immutable variable `y` and mutable variable `x`, both with the value 5.
/// Prints their values to show that `x` can be mutated but `y` cannot.
/// Also defines a constant `THREE_HOURS_IN_SECONDS` and prints its value.
fn main() {
    println!("\nTesting Mutability");

    let mut x = 5; // mutable

    let y = 5; // immutable

    println!("The value of x is: {x} {y}");

    x = 5;

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("here is a constant: {THREE_HOURS_IN_SECONDS}");
}
