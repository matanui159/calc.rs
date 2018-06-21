use std::iter::Peekable;
use std::str::Chars;

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
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Result<Token, String>;

	fn next(&mut self) -> Option<Result<Token, String>> {
		let c = *(&mut self.input).skip_while(|&c| {
			c.is_whitespace()
		}).peekable().peek()?;
		assert_eq!(c, *self.input.peek()?);

		if "+-*/%^()|".contains(c) {
			self.input.next();
			Some(Ok(Token::Op(c)))
		} else {
			Some(Err(format!("Unknown symbol {:?}", c)))
		}
	}
}