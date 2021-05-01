use std::{
    collections::HashMap,
    convert::TryFrom,
    io
};

/*
Given a list of integers,
    use a vector and return the mean (the average value),
    median (when sorted, the value in the middle position),
    and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

fn main() {

    println!("Enter space seperated integers");

    loop {
        let mut number_input = String::new();

        io::stdin().read_line(&mut number_input).expect("Invalid input given!");

        let number_input = number_input.trim();

        if number_input == "quit" {
            break
        }

        let mut list: Vec<i128> = number_input.split_whitespace().map(|string_num| string_num.parse().unwrap()).collect();

        let mean: usize = usize::try_from(list.iter().sum::<i128>()).unwrap() / list.len();
        list.sort();
        let median = list[list.len() / 2];

        let mut numbers = HashMap::new();

        for num in &list {
            let count = numbers.entry(num).or_insert(0);
            *count += 1;
        }

        println!("mean: {}\nmedian: {}\nnumbers: {:#?}", mean, median, numbers);
    }
}
