/// This function prints some statements to test statements and expressions,
/// assigns the result of an expression to y, prints y, calls get_val() and
/// prints the result.
fn main() {
    println!("Testing Statement and Expressions");

    // Assign the result of an expression to y
    let y = {
        let x = 3; // Statement, returns nothing
        x + 1 // Expression, returns x + 1
    };

    println!("the value of y from an expression: {y}");

    let x = get_val();
    println!("x = {x}");
}

/// Prints a message, returns 5
fn get_val() -> i32 {
    println!("Testing Functions with Return Values");
    return 5;
}
