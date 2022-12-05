use std::io;

fn main() {
    println!("Enter a numerical temperature.");

    let mut input_temperature = String::new();

    io::stdin()
        .read_line(&mut input_temperature)
        .expect("Failed to read line");

    println!("Units of measure:");
    println!("1. Farenheit");
    println!("2. Celcius");

    let mut units = String::new();

    io::stdin()
        .read_line(&mut units)
        .expect("Failed to read line");

    let input_temperature: u64 = match input_temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let units: u32 = match units.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if units == 1 {
        println!("your input was: {input_temperature}F");
        //temperature converted to celcius
        let temp = input_temperature - 32;
        let temp = temp * 5 / 9;
        println!("{input_temperature}F = {temp}C");
    } else if units == 2 {
        println!("your input was: {input_temperature}C");
        //temperature converted to farenheit
        let temp = input_temperature * 9 / 5;
        let temp = temp + 32;
        println!("{input_temperature}C = {temp}F");
    } else {
        println!("unrecognized reading. abort!");
    }
}
