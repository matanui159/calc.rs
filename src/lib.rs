pub mod lexer;
use lexer::*;
type Result<T> = std::result::Result<T, String>;

fn parse_value(lexer: &mut Lexer) -> Result<f64> {
	match lexer.next_result()? {
		Token::Num(value) => Ok(value),
		token => token.unexpected()
	}
}

fn parse_brackets(lexer: &mut Lexer) -> Result<f64> {
	match lexer.peek_result()? {
		Token::Op('(') => {
			lexer.next();
			let result = parse_expression(lexer)?;
			match lexer.next_result()? {
				Token::Op(')') => Ok(result),
				token => token.unexpected()
			}
		},
		_ => parse_value(lexer)
	}
}

fn parse_abs(lexer: &mut Lexer) -> Result<f64> {
	match lexer.peek_result()? {
		Token::Op('|') => {
			lexer.next();
			let result = parse_expression(lexer)?;
			match lexer.next_result()? {
				Token::Op('|') => Ok(result.abs()),
				token => token.unexpected()
			}
		},
		_ => parse_brackets(lexer)
	}
}

fn parse_unary(lexer: &mut Lexer) -> Result<f64> {
	match lexer.peek_result()? {
		Token::Op('+') => {
			lexer.next();
			parse_abs(lexer)
		},
		Token::Op('-') => {
			lexer.next();
			Ok(-parse_abs(lexer)?)
		},
		_ => parse_abs(lexer)
	}
}

fn parse_expression(lexer : &mut Lexer) -> Result<f64> {
	parse_unary(lexer)
}

pub fn parse(input: &str) -> Result<f64> {
	let mut lexer = Lexer::new(input);
	let value = parse_expression(&mut lexer)?;
	match lexer.next() {
		Some(Ok(token)) => token.unexpected(),
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
	#[should_panic(expected = "Unexpected end")]
	fn error_none() {
		parse_test("", 0.0)
	}

	#[test]
	#[should_panic(expected = "Unexpected token 2")]
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
	#[should_panic(expected = "Unexpected end")]
	fn error_brackets() {
		parse_test("(3", 0.0)
	}

	#[test]
	fn abs_simple() {
		parse_test("|3|", 3.0)
	}

	#[test]
	#[should_panic(expected = "Unexpected end")]
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
	#[should_panic(expected = "Unexpected token \\'-\\'")]
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
		parse_test("2.0 + 2.1 * 2.2", 4.96)
	}

	#[test]
	fn brackets() {
		parse_test("2.3 * -(2.4 + 2.5)", -11.27)
	}

	#[test]
	fn averge() {
		parse_test("(1 + 2 + 3) / 3", 2.0)
	}
}