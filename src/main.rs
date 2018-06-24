extern crate calc;

use std::io;
use std::io::Write;

fn main() {
	let mut input = String::new();
	loop {
		print!(">> ");
		io::stdout().flush().unwrap();

		input.clear();
		io::stdin().read_line(&mut input).unwrap();

		match calc::parse(input.as_str()) {
			Ok(value) => println!("= {}", value),
			Err(error) => eprintln!("{}", error)
		}
	}
}
