use std::io;

fn get_fibonacci(num: i128) -> i128 {
	if num == 0 || num == 1 {
		num
	} else {
		get_fibonacci(num - 1) + get_fibonacci(num - 2)
	}
}

fn main() {

    loop {
        println!("Please enter the index to get the fibonacci number for");

        let mut at_index = String::new();

        io::stdin()
            .read_line(&mut at_index)
            .expect("Failed to read line");

        let at_index = at_index.trim();

        if at_index == "quit" {
            break;
        }

        let at_index: i128 = match at_index.parse() {
            Ok(num) => num,
            Err(err) => {
            	println!("{}", err);
            	continue;
            },
        };

        println!("Fibonacci Number {}", get_fibonacci(at_index))
    }
}
