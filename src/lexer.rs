use super::{ParseError, Result};
use std::iter::Peekable;
use std::str::Chars;
use std::fmt::{Display, Formatter, Error};
use std::result;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token {
	Num(f64),
	Op(char)
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter) -> result::Result<(), Error> {
		match self {
			Token::Num(value) => write!(f, "{}", value),
			Token::Op(value) => write!(f, "'{}'", value)
		}
	}
}



pub struct Lexer<'a> {
	input: Peekable<Chars<'a>>,
	peeked: Option<Result<Token>>
}

impl<'a> Lexer<'a> {
	pub fn new(input: &str) -> Lexer {
		Lexer {
			input: input.chars().peekable(),
			peeked: None
		}
	}

	fn read_number(&mut self) -> Result<Token> {
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
					return Err(ParseError::UnexpectedSymbol('.'))
				}
			} else {
				break
			}
		}

		Ok(Token::Num(if den == 0.0 {
			num
		} else {
			num / den
		}))
	}

	pub fn peek(&mut self) -> Option<Result<Token>> {
		if self.peeked == None {
			while self.input.peek()?.is_whitespace() {
				self.input.next();
			}
			let c = *self.input.peek()?;

			self.peeked = Some(if "()|^*/+-".contains(c) {
				self.input.next();
				Ok(Token::Op(c))
			} else if c.is_digit(10) || c == '.' {
				self.read_number()
			} else {
				Err(ParseError::UnknownSymbol(c))
			});
		}
		self.peeked
	}

	pub fn peek_result(&mut self) -> Result<Token> {
		self.peek().ok_or(ParseError::UnexpectedEnd)?
	}

	pub fn next_result(&mut self) -> Result<Token> {
		let result = self.peek_result();
		self.peeked = None;
		result
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Result<Token>;
	fn next(&mut self) -> Option<Result<Token>> {
		let option = self.peek();
		self.peeked = None;
		option
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
		lexer_test("()| ^ */ +-", vec![
			Token::Op('('),
			Token::Op(')'),
			Token::Op('|'),
			Token::Op('^'),
			Token::Op('*'),
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
	#[should_panic(expected = "UnknownSymbol('!')")]
	fn error_unknown() {
		lexer_test("!", vec![])
	}

	#[test]
	#[should_panic(expected = "UnexpectedSymbol('.')")]
	fn error_unexpected() {
		lexer_test("1.2.3", vec![])
	}

	#[test]
	fn peek() {
		let mut lexer = Lexer::new("1");
		lexer.peek();
		assert_eq!(lexer.next(), Some(Ok(Token::Num(1.0))));
	}
}