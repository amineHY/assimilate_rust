fn ownership() {
    println!("\n[Testing] ownership");

    let s1 = String::from("hello"); // s1 is the owner of this string
    let s2 = s1.clone(); // s2 gets its own copy of the data from `s1`
                         // println!("y: {}", s2);
    println!("{}", s2);
}

fn memory_safety() {
    println!("\n[Testing] memory_safety");

    let mut v = vec![1, 2, 3];
    println!("The length of the vector is: {}", v.len()); // The length of the vector is: 3
    v[2] = 4;
    println!("v[2] {}", v[2]); // The length of the vector is: 3
    println!("v[1] {}", v[1]); // The length of the vector is: 3
}

fn immutability() {
    println!("\n[Testing] immutability");

    let x = 5; // x is immutable by default
    println!("{}", x); // This works fine

    let mut y = 10; // You can explicitly make a variable mutable
    y += 2; // Now this line of code will work
}

fn pattern_matching() {
    println!("\n[Testing] pattern_matching");

    let x = Some(5);
    match x {
        Some(value) => println!("{}", value),
        None => (),
    }
}

fn add(x: i32, y: i32) -> i32 {
    println!("\n[Testing] addition");
    let sum = x + y; // Declare and initialize 'sum' variable
    println!("{} + {} = {}", x, y, sum);
    sum // Return the result
}

fn data_structures() {
    println!("\n[Testing] data_structures");

    let mut v = vec![1, 2, 3]; // Create a mutable vector
    v.push(4); // Add an element to the end of the vector
    println!("{}", v[0]); // Access elements using indexing
}

fn control_flow() {
    println!("\n[Testing] control_flow");

    let number = 3;

    if number < 5 {
        println!("less than 5");
    } else {
        println!("equal to or greater than 5");
    }
}

// fn error_Handling() -> Result<(), Box<dyn std::error::Error>> {
//     // This function returns a result type
//     let file_result = std::fs::read_to_string("hello.txt"); // Try to read the file

//     match file_result {
//         // Match on the result value
//         Ok(contents) => println!("{}", contents), // If `Ok`, we get the contents of the file
//         Err(error) => {
//             // If `Err`, we get an error type
//             eprintln!("There was an error: {}", error);
//             Ok(()) // Return a value from the function
//         }
//     };
// }

use std::sync::{Arc, Mutex}; // Import the necessary concurrent data structures from the standard library
use std::thread; // We also need to import thread functionality

// fn concurrency() {
//     let mut counter = Arc::new(Mutex::new(0)); // Create a shared and thread-safe counter

//     let mut handles = vec![]; // Vec to store threads

//     for _ in 0..10 {
//         // Spawn multiple threads that increment the counter
//         let counter = Arc::clone(&counter); // Clone the reference to the counter
//         let handle = thread::spawn(move || {
//             // Create a new thread and store its handle
//             let mut num = counter.lock().unwrap(); // Lock the mutex, get the value from it, then unlock it
//             *num += 1; // Increment the value
//         });
//         handles.push(handle); // Push the handle to a Vec so we can join them later
//     }

//     for handle in handles {
//         // Wait for all threads to finish
//         handle.join().unwrap(); // Join and handle any errors that might occur
//     }

//     println!("The counter's final value is {}", *counter.lock().unwrap()); // Print the final value of the counter
// }

fn main() {
    ownership();
    memory_safety();
    immutability();
    pattern_matching();
    add(5, 6);
    control_flow();
    // error_Handling();
    data_structures();
    // concurrency();
}
