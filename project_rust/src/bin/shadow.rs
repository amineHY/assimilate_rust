/// Demonstrates variable shadowing in Rust.
///
/// Declares a variable `g` and initializes it to 5. Prints out the value.
/// Then re-declares `g` and assigns it to `g + 10`, shadowing the previous
/// value. Prints out the new value.
///
/// Creates an inner scope and declares another `g` shadowing the outer one,
/// assigning it to `g * 2`. Prints the value in this inner scope.
///
/// After the inner scope ends, prints `g` again, showing it has the value
/// from the second declaration in the outer scope.
fn main() {
    println!("\nTesting Shadowing");
    let g = 5;
    println!("the value of g is: {g}");

    // let's shadow the previous value of x
    let g = g + 10;
    println!("the value of g + 10 is: {g}");

    {
        let g = g * 2;
        println!("the value of g * 2 in the inner scope is: {g}");
    }
    println!("the value of g is: {g}");
}
