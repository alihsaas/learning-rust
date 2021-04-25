use std::io;

fn fah_to_cel(tem: i16) -> i16 {
	(tem - 32) * 5/9
}

fn cel_to_fah(tem: i16) -> i16 {
	(tem * 9/5) + 32
}

fn main() {

	loop {
		println!("Fahrenheit (F) or Celsius (C)");

		let mut scale = String::new();

		io::stdin()
			.read_line(&mut scale)
			.expect("Invalid scale passed");

		let scale = scale.trim();

		if scale == "quit" {
			break;
		} else if scale != "F" && scale != "C" {
			panic!("Invalid scale passed")
		}

		println!("Enter Temperature");

		let mut input_temperature = String::new();

		io::stdin()
			.read_line(&mut input_temperature)
			.expect("Invalid input passed");

		let input_temperature: i16 = match input_temperature.trim().parse() {
			Ok(num) => num,
			Err(err) => {
				println!("{}", err);
				continue;
			},
		};

		let result = if scale == "F" {
			format!("{}°F", fah_to_cel(input_temperature))
		} else { 
			format!("{}°C", cel_to_fah(input_temperature))
		};

		println!("{}", result);

	}
}
