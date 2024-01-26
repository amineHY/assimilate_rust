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
