use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    println!("Guess a number 1-100");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please enter your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //shadowed - often used to convert a value from one type to another type.
        // trim -   trim on a string removes any whitespace at the beginning and end
        //          enter adds a newline as well

        // parse - converts type specified by left hand side of equation
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            cmp::Ordering::Less => println!("Too small"),
            cmp::Ordering::Greater => println!("Too big"),
            cmp::Ordering::Equal => {
                println!("Excellent! You win!");
                break;
            }
        }
    }
}
