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

    let mut sequence: [i128; 1000] = [0; 1000];
    //make an array
    for number in 0..n {
        if number < 2 {
            sequence[number] = number as i128;
        } else {
            sequence[number] = sequence[number - 1] + sequence[number - 2];
        }
        let index = sequence[number];
        println!("number: {number}, value: {index}");
    }
}

// fibonacci gets really big really quickly
// 91million - lets say 100 million     =                 100,000,000
//                      354 quintillion = 354,224,848,179,261,915,075
