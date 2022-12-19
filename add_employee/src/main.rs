// using a hash map and vectors -
// create a text interface to allow a user to add employee
// names to a department in a company.
// ex:
// add sally to engineering
// add amir to sales
// then
// let user retrieve a list of all people in a department
// OR all people in company by department, sorted alphabetically

use std::collections::HashMap;
use std::io;

fn main() {
    let employee_data = &mut HashMap::new();

    loop {
        println!("select an operation: ");
        println!("1. add employee to directory");
        println!("2. query for all employees");
        println!("3. exit");
        let mut selected_op = String::new();
        io::stdin()
            .read_line(&mut selected_op)
            .expect("Failed to read line");
        // validates input: if invalid return 0 (which exits the program below)
        let selected_op: u32 = match selected_op.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        match selected_op {
            1 => add_employee(employee_data),
            2 => sort_directory(employee_data),
            _ => break,
        }
    }
}

fn add_employee(employee_data: &mut HashMap<String, String>) {
    println!("add employee to department: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // chop off the word 'add ' from the front of the string
    let modified_input = &input[4..input.len()];
    // parse name
    let name = parser(modified_input, "name");
    // parse department
    let department = parser(modified_input, "department");
    // add the info to the hash map
    employee_data.insert(String::from(name), String::from(department));
}

fn parser<'a>(input: &'a str, property_to_parse: &'a str) -> &'a str {
    let property = input;
    let parsed_property = match property_to_parse {
        "name" => parse_name(property),
        _ => parse_department(property),
    };
    parsed_property
}

fn parse_name(property: &str) -> &str {
    let mut end_index = 0;
    let mut count = 0;
    // parse name
    for letter in property.chars() {
        if letter.to_string() == " " {
            end_index = count;
            break;
        }
        count += 1;
    }
    let name = &property[0..end_index];
    name
}

fn parse_department(property: &str) -> &str {
    let mut start_index = 0;
    let mut count_spaces = 0;
    let mut count = 0;
    // parse department
    for letter in property.chars() {
        if letter.to_string() == " " {
            count_spaces += 1;

            if count_spaces > 1 {
                start_index = count + 1;
                break;
            }
        }
        count += 1;
    }

    let department = &property[start_index..property.len() - 1];
    department
}

fn sort_directory(employee_data: &mut HashMap<String, String>) {
    let mut hash_to_vec: Vec<(&String, &String)> = employee_data.iter().collect();
    hash_to_vec.sort();
    println!("directory: {:?}", hash_to_vec);
}
