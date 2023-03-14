use std::io;

fn main() {
/*
    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
*/

    // Addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");

    // Subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    // Multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");

    // Division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("The value of quotient is: {quotient}");
    println!("The value of truncated is: {truncated}");

    // Remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");

    // Bool
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("The value of t is: {t}");
    println!("The value of f is: {f}");

    // Char
    let w = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("The value of w is: {w}");
    println!("The value of z is: {z}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    // The Tuple Type
    let tup = (500, 6.4, 1);
    let (x, _y, _c) = tup;
    println!("The value of x is: {x}");

    {

    println!("Shadowing ...");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!("The value of five_hundred is: {five_hundred}");

    let six_point_four = x.1;
    println!("The value of six_point_four is: {six_point_four}");

    let one = x.2;
    println!("The value of one is: {one}");

    //println!("{x}"); ^ `(i32, f64, u8)` cannot be formatted with the default formatter

    }

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let january = months[0];
    
    println!("months[0] = {january}");



}