use std::io;

fn main() {
    println!("Generate the nth Fibonacci number.");
    println!("Enter a value for n: ");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    println!("The value you entered is: {n}");

    let mut sequence: [usize; 1000] = [0; 1000];
    //make an array
    for number in 0..n {
        if number < 2 {
            sequence[number] = number;
        } else {
            sequence[number] = sequence[number - 1] + sequence[number - 2];
        }
        let index = sequence[number];
        println!("number: {number}, value: {index}");
    }
}
