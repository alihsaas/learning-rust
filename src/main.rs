use std::io;


const VOWELS: [char; 5] = ['a', 'o', 'e', 'u', 'i'];


fn main() {

    println!("Enter a word!");

    loop {
        let mut word_input = String::new();

        io::stdin().read_line(&mut word_input).expect("Invalid input passed!");

        let world_input = word_input.trim();

        if world_input == "quit" {
            break
        }

        let word = world_input.split_whitespace().next().unwrap();
        let mut chars = word.chars();

        if let Some(c) = chars.next() {
            let pig_latin = if !VOWELS.contains(&c) {
                format!("{}-{}ay", chars.map(|char| char.to_string()).collect::<String>(), c)
            } else {
                format!("{}-hay", word)
            };

            println!("{}", pig_latin);
        }
    }
}
