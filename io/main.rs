// Import necessary libraries
use std::io;

// Define a function that takes input, performs a computation, and returns a result
fn calculate_square(input: i32) -> i32 {
    let square = input * input;
    square // Implicit return without a semicolon makes it the return value
}

// Define the main function
fn main() {
    // Print a message to the console
    println!("Welcome to the Rust Introduction!");

    // Declare a mutable variable
    let mut user_input = String::new();

    // Prompt the user for input
    println!("Please enter a number:");

    // Read input from the user
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    // Convert the input to an integer (unwrap is used here for simplicity)
    let number: i32 = user_input.trim().parse().unwrap();

    // Call a function and store the result in a variable
    let squared_number = calculate_square(number);

    // Print the result
    println!("The square of {} is: {}", number, squared_number);
}
