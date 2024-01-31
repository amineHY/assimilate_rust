/// Prints 'Testing Control Flow' and then checks if a variable is less than 5.
/// If so, prints 'condition is true', otherwise prints 'condition is false'.
fn main() {
    println!("Testing Control Flow");

    let number = 3;
    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }
}
