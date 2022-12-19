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
    let mut employee_data = HashMap::new();

    loop {
        println!("select an operation: ");
        println!("1. add employee to directory");
        println!("2. query for all employees");
        println!("3. exit");
        let mut selected_op = String::new();
        io::stdin()
            .read_line(&mut selected_op)
            .expect("Failed to read line");
        let selected_op: u32 = match selected_op.trim().parse() {
            Ok(num) => num,
            Err(_) => 3,
        };

        if selected_op == 3 {
            println!("exiting");
            break;
        } else if selected_op == 1 {
            println!("add employee to department: ");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            add_employee(&input, &mut employee_data);
        } else if selected_op == 2 {
            let mut hash_to_vec: Vec<(&String, &String)> = employee_data.iter().collect();
            hash_to_vec.sort();

            println!("employee data: {:?}", hash_to_vec);
        } else {
            println!("unsupported operation");
            break;
        }
    }
}

fn add_employee(input: &str, employee_data: &mut HashMap<String, String>) {
    let modified_input = &input[4..input.len()];
    // parse name
    let name = parser(modified_input, "name");
    // parse department
    let department = parser(modified_input, "department");

    // now you are at the step of storing - hash map and vectors
    // let mut employee_data = HashMap::new();
    employee_data.insert(String::from(name), String::from(department));
}

fn parser<'a>(input: &'a str, property_to_parse: &'a str) -> &'a str {
    let mut property = input;

    let mut count = 0;

    if property_to_parse == "name" {
        let mut end_index = 0;
        // parse name
        for letter in property.chars() {
            if letter.to_string() == " " {
                end_index = count;
                break;
            }
            count += 1;
        }
        property = &property[0..end_index];
        property
    } else if property_to_parse == "department" {
        let mut start_index = 0;
        let mut count_spaces = 0;

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

        property = &property[start_index..property.len() - 1];
        property
    } else {
        property = "please provide a property";
        property
    }
}
