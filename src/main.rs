use std::{collections::HashMap, io};

/*
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
For example, “Add Sally to Engineering” or “Add Amir to Sales.”
Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/

enum Request {
    Add,
    Remove,
}

fn main() {
    println!("Add <name> to <rank>\nRemove <name>");

    let mut employees = HashMap::new();

    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Invalid input passed!");

        let user_input = user_input.trim().to_lowercase();

        match user_input.as_str() {
            "quit" => break,
            "list employees" => {
                let mut employee_names = employees.keys().collect::<Vec<_>>();
                employee_names.sort();
                if employee_names.is_empty() {
                    println!("No employees");
                } else {
                    for name in employee_names {
                        println!("{} : {}", &name, &employees.get(name).unwrap());
                    }
                }
                continue;
            }
            _ => (),
        }

        let mut words = user_input.split_whitespace();
        let request: Request = match words.next() {
            Some(request) => match request {
                "add" => Request::Add,
                "remove" => Request::Remove,
                _ => {
                    println!("Invalid Request passed use Add or Remove instead");
                    continue;
                }
            },
            None => {
                println!("No request passed Add | Remove");
                continue;
            }
        };
        let name = match words.next() {
            Some(name) => name,
            None => {
                println!("No name passed!");
                continue;
            }
        };

        match request {
            Request::Add => employees.insert(
                name.to_string().clone(),
                match words.nth(1) {
                    Some(rank) => {
                        println!("Added {} to {}!", &name, &rank);
                        rank.to_string().clone()
                    }
                    None => {
                        println!("No rank passed!");
                        continue;
                    }
                },
            ),
            Request::Remove => {
                println!("Removed {}", &name);
                employees.remove(&name.to_string())
            }
        };
    }
}
