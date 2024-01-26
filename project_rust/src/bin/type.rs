fn main() {
    println!("\nTesting Types:");

    sum(5, 12);
    sub(5, 12);
    prod(5, 12);
    div(-5.0, 3.0);
    div(56.7, 32.2);
    set_bool(true, false);
    set_string('z', 'Z', '😻');
    print_labeled_measurement(5, 'h');

    // remainder
    let remainder = 43 % 5;

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("{guess}, {six_point_four}");
    array();
}

fn array() {
    println!("Array : fixed size and same type");
    let v = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // equivalent let a = [3, 3, 3, 3, 3];

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}

// Math

fn sum(x: i32, y: i32) {
    println!("Addition");
    let sum = x + y;
    println!("{x} + {y} = {sum}")
}

fn sub(x: i32, y: i32) {
    println!("Subtraction");
    let difference = x - y;
    println!("{x} - {y} = {difference}")
}

fn prod(x: i32, y: i32) {
    println!("Multiplication");
    let product = x * y;
    println!("{x} * {y} = {product}")
}

fn div(x: f64, y: f64) {
    println!("Division");
    let division = x / y;
    println!("{x} / {y} = {division}")
}

// other types
fn set_bool(t: bool, f: bool) {
    println!("Set Boolean");
    println!("{t}, {f}")
}

fn set_string(c: char, z: char, emojis: char) {
    println!("Set String");
    println!("{c}, {z}, {emojis}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("Print labeled measurement");
    println!("the measurement is: {value}{unit_label}")
}
