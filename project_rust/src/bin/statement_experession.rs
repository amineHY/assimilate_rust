fn main() {
    println!("Testing Statement and Expressions");
    let y = {
        let x = 3; // this is a statement, it return nothing
        x + 1
    }; // this is an expression; it returns x + 1
    println!("the value of y from an expression: {y}");

    let x = get_val();
    println!("x = {x}")
}

fn get_val() -> i32 {
    println!("Testing Functions with Return Values");
    5
}
