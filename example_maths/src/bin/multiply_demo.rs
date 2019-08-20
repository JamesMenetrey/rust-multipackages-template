//! executables can also be created in src/bin for quick demos.

use std::io::{stdin, stdout, Write};
use example_maths::multiply;

const PARSING_ERROR: &str = "Cannot parse the number as an unsigned 64-bit integer.";

fn main() {
    // Ask the first number
    print!("Enter the first number: ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let x = input.trim().parse().expect(PARSING_ERROR);

    // As the second number
    print!("Enter the second number: ");
    stdout().flush().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let y = input.trim().parse().expect(PARSING_ERROR);

    // Call the API of the package
    println!();
    println!("The result of {} * {} is {}.", x, y, multiply(x, y));
}