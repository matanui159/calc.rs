extern crate calc;

use calc::lexer::*;

fn main() {
	let lexer = Lexer::new("$+-/");
	for result in lexer {
		match result {
			Ok(token) => {
				match token {
					Token::Op(value) => println!("{}", value),
					Token::Num(value) => println!("{}", value)
				}
			},
			Err(error) => {
				eprintln!("{}", error);
				break;
			}
		}
	}
}
