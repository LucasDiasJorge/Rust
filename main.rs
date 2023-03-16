fn main() {
    
    another_function(5);
    print_labeled_measurement(5, 'h');

    let x = five();

    println!("The value of x is: {x}");

    let y = plus_one(5);

    println!("The value of y is: {y}");

}

//Parameters
fn another_function(z: i32) {
    println!("The value of z is: {z}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Functions with Return Values
fn five() -> i32 {
    return 5;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}