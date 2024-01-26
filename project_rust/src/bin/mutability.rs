fn main() {
    println!("\nTesting Mutability");
    let mut x = 5; // mutable
    let y = 5; // immutable
    println!("The value of x is: {x} {y}");
    x = 5;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("here is a constant: {THREE_HOURS_IN_SECONDS}");
}
