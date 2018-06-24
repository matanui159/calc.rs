pub mod lexer;
use lexer::*;
use std::fmt::{Display, Formatter, Error};
use std::result;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ParseError {
	UnexpectedEnd,
	UnknownSymbol(char),
	UnexpectedSymbol(char),
	UnexpectedToken(Token)
}

impl Display for ParseError {
	fn fmt(&self, f: &mut Formatter) -> result::Result<(), Error> {
		match self {
			ParseError::UnexpectedEnd => write!(f, "Unexpected end"),
			ParseError::UnknownSymbol(c) => write!(f, "Unknown symbol '{}'", c),
			ParseError::UnexpectedSymbol(c) => write!(f, "Unexpected symbol '{}'", c),
			ParseError::UnexpectedToken(token) => write!(f, "Unexpected token {}", token)
		}
	}
}

type Result<T> = result::Result<T, ParseError>;



fn parse_value(lexer: &mut Lexer) -> Result<f64> {
	Ok(match lexer.peek_result()? {
		Token::Op('+') => {
			lexer.next();
			1.0
		},
		Token::Op('-') => {
			lexer.next();
			-1.0
		},
		_ => 1.0
	} * match lexer.peek_result()? {
		Token::Op('(') => {
			lexer.next();
			let value = parse_expression(lexer)?;
			match lexer.next_result()? {
				Token::Op(')') => value,
				token => return Err(ParseError::UnexpectedToken(token))
			}
		},
		Token::Op('|') => {
			lexer.next();
			let value = parse_expression(lexer)?;
			match lexer.next_result()? {
				Token::Op('|') => value.abs(),
				token => return Err(ParseError::UnexpectedToken(token))
			}
		},
		Token::Num(value) => {
			lexer.next();
			value
		},
		token => return Err(ParseError::UnexpectedToken(token))
	})
}

fn parse_pow(lexer: &mut Lexer) -> Result<f64> {
	let mut value = parse_value(lexer)?;
	if let Some(result) = lexer.peek() {
		match result? {
			Token::Op('^') => {
				lexer.next();
				value = value.powf(parse_pow(lexer)?);
			},
			_ => ()
		}
	}
	Ok(value)
}

fn parse_product(lexer: &mut Lexer) -> Result<f64> {
	let mut value = parse_pow(lexer)?;
	while let Some(result) = lexer.peek() {
		match result? {
			Token::Op('*') => {
				lexer.next();
				value *= parse_pow(lexer)?;
			},
			Token::Op('/') => {
				lexer.next();
				value /= parse_pow(lexer)?;
			},
			_ => break
		}
	}
	Ok(value)
}

fn parse_expression(lexer: &mut Lexer) -> Result<f64> {
	let mut value = parse_product(lexer)?;
	while let Some(result) = lexer.peek() {
		match result? {
			Token::Op('+') => {
				lexer.next();
				value += parse_product(lexer)?;
			},
			Token::Op('-') => {
				lexer.next();
				value -= parse_product(lexer)?;
			},
			_ => break
		}
	}
	Ok(value)
}

pub fn parse(input: &str) -> Result<f64> {
	let mut lexer = Lexer::new(input);
	let value = parse_expression(&mut lexer)?;
	match lexer.next() {
		Some(Ok(token)) => Err(ParseError::UnexpectedToken(token)),
		Some(Err(error)) => Err(error),
		None => Ok(value)
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	fn parse_test(input: &str, expect: f64) {
		assert_eq!(parse(input).unwrap(), expect);
	}

	#[test]
	#[should_panic(expected = "UnexpectedEnd")]
	fn error_none() {
		parse_test("", 0.0)
	}

	#[test]
	#[should_panic(expected = "UnexpectedToken(Num(2.0))")]
	fn error_unexpected() {
		parse_test("1 2", 0.0)
	}

	#[test]
	fn value_simple() {
		parse_test("1", 1.0)
	}

	#[test]
	fn brackets_simple() {
		parse_test("(2)", 2.0)
	}

	#[test]
	#[should_panic(expected = "UnexpectedEnd")]
	fn error_brackets() {
		parse_test("(3", 0.0)
	}

	#[test]
	fn abs_simple() {
		parse_test("|3|", 3.0)
	}

	#[test]
	#[should_panic(expected = "UnexpectedEnd")]
	fn error_abs() {
		parse_test("|4", 4.0)
	}

	#[test]
	fn unary_pos() {
		parse_test("+4", 4.0)
	}

	#[test]
	fn unary_neg() {
		parse_test("-5", -5.0)
	}

	#[test]
	#[should_panic(expected = "UnexpectedToken(Op('-'))")]
	fn error_double_unary() {
		parse_test("--4", 0.0)
	}

	#[test]
	fn abs() {
		parse_test("|-6|", 6.0)
	}

	#[test]
	fn pow() {
		parse_test("7 ^ 2", 49.0)
	}

	#[test]
	fn mul() {
		parse_test("8 * 9", 72.0)
	}

	#[test]
	fn div() {
		parse_test("1.32 / 1.1", 1.2)
	}

	#[test]
	fn add() {
		parse_test("1.3 + 1.4", 2.7)
	}

	#[test]
	fn sub() {
		parse_test("3.1 - 1.5", 1.6)
	}

	#[test]
	fn order1() {
		parse_test("1.7 * 1.8 + 1.9", 4.96)
	}

	#[test]
	fn order2() {
		parse_test("2.0 + 2.1 * 3.3", 8.93)
	}

	#[test]
	fn brackets() {
		parse_test("2.3 * -(2.4 + 2.5)", -11.27)
	}

	#[test]
	fn averge() {
		parse_test("(1 + 2 + 3) / 3", 2.0)
	}

	#[test]
	fn add_neg() {
		parse_test("4 + -5", -1.0)
	}
}