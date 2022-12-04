use std::io;

fn main() {
    // variables and mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // data types
    // addtition
    let sum = 5 + 10;
    println!("sum = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("subtraction = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("multiplication = {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("division = {quotient}");
    let floored = 2 / 3; // results in 0
    println!("floored = {floored}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder = {remainder}");

    // tuple
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("the value of x is {x}");
    println!("the value of y is {y}");
    println!("the value of z is {z}");

    let five_hundred = tup.0;
    println!("the value of x is {five_hundred}");
    let second_item_in_tup = tup.1;
    println!("the value of y is {second_item_in_tup}");

    // guessing game 2
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index.trim().parse().expect("Index entered is not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
