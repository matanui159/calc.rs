use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
	Num(f64),
	Op(char)
}

pub struct Lexer<'a> {
	input: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
	pub fn new(input: &str) -> Lexer {
		Lexer {
			input: input.chars().peekable()
		}
	}

	fn read_number(&mut self) -> Result<f64, String> {
		let mut num = 0.0;
		let mut den = 0.0;
		while let Some(&c) = self.input.peek() {
			if let Some(d) = c.to_digit(10) {
				self.input.next();
				num = num * 10.0 + d as f64;
				den *= 10.0;
			} else if c == '.' {
				self.input.next();
				if den == 0.0 {
					den = 1.0;
				} else {
					return Err(format!("Unexpected symbol '.'"))
				}
			} else {
				break
			}
		}
		if den == 0.0 {
			Ok(num)
		} else {
			Ok(num / den)
		}
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Result<Token, String>;

	fn next(&mut self) -> Option<Result<Token, String>> {
		while self.input.peek()?.is_whitespace() {
			self.input.next();
		}
		let c = *self.input.peek()?;

		if "()|e^*/\\+-".contains(c) {
			self.input.next();
			Some(Ok(Token::Op(match c {
				'\\' => '/',
				c => c
			})))
		} else if c.is_digit(10) || c == '.' {
			match self.read_number() {
				Ok(value) => Some(Ok(Token::Num(value))),
				Err(error) => Some(Err(error))
			}
		} else {
			Some(Err(format!("Unknown symbol '{}'", c)))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn lexer_test(input: &str, expect: Vec<Token>) {
		let result: Vec<_> = Lexer::new(input)
			.map(|result| result.unwrap()).collect();
		assert_eq!(result, expect);
	}

	#[test]
	fn symbols() {
		lexer_test("()| ^ */\\ +-", vec![
			Token::Op('('),
			Token::Op(')'),
			Token::Op('|'),
			Token::Op('^'),
			Token::Op('*'),
			Token::Op('/'),
			Token::Op('/'),
			Token::Op('+'),
			Token::Op('-')
		]);
	}

	#[test]
	fn numbers() {
		lexer_test("1 23 4.5 67.89", vec![
			Token::Num(1.0),
			Token::Num(23.0),
			Token::Num(4.5),
			Token::Num(67.89)
		]);
	}

	#[test]
	fn symbols_and_numbers() {
		lexer_test("23 + 67.89", vec![
			Token::Num(23.0),
			Token::Op('+'),
			Token::Num(67.89)
		]);
	}

	#[test]
	#[should_panic]
	fn error_unknown_symbol() {
		lexer_test("!", vec![])
	}

	#[test]
	#[should_panic]
	fn error_unexpected_symbol() {
		lexer_test("1.2.3", vec![])
	}
}