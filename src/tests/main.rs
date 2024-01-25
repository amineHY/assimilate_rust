// use csv::ReaderBuilder;
// use serde_derive::Deserialize;
// use std::error::Error;
// use std::fs::File;

// // Define a struct to represent your CSV data
// #[derive(Debug, Deserialize)]
// struct Record {
//     // Define fields matching the columns in your CSV file
//     name: String,
//     age: u32,
//     // Add more fields as needed
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     // Open the CSV file
//     let file = File::open("sample.csv")?;
//     // Create a CSV reader with flexible settings
//     let mut rdr = ReaderBuilder::new().flexible(true).from_reader(file);

//     // Iterate over the CSV records and print the data
//     for result in rdr.deserialize::<Record>() {
//         // Handle each CSV record
//         match result {
//             Ok(record) => {
//                 // Access fields of the record and print data
//                 println!("Name: {}, Age: {}", record.name, record.age);
//             }
//             Err(err) => eprintln!("Error parsing record: {}", err),
//         }
//     }

//     Ok(())
// }
