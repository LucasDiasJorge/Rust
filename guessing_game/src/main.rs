use std::io;

fn main() {

    println!("Guess the number!");

    println!("Please, input your guess.");

    let mut guess = String::new();

    println!("Your guessed: {guess}");

    let apples = 5; // immutable
    let mut bananas = 5; // mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("apples = {apples} and bananas = {bananas}");

    println!("apple + bananas = {}", apples + bananas);

}