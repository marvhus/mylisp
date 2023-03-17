use std::iter::Peekable;
use std::fmt;

#[derive(Debug)]
pub enum KeywordKind {
	Defun,
}

#[derive(Debug)]
pub enum TokenKind {
	Plus,
	Minus,

	Keyword(KeywordKind),
	Ident(String),

	Int(i64),
	
	OpenParan,
	CloseParan,
}

pub struct Token {
	pub	line: i64,
	pub	col:  i64,

	pub	kind: TokenKind,
}

impl Token {
	fn new(line: i64, col: i64, kind: TokenKind) -> Self {
		Self { line, col, kind }
	}
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}:{}, {:?})", self.line, self.col, self.kind)
	}
}

pub struct Lexer<Chars: Iterator<Item=char>> {
	chars: Peekable<Chars>,
	line: i64,
	col:  i64
}

impl<Chars: Iterator<Item=char>> Lexer<Chars> {
	pub fn from_iter(chars: Chars) -> Self {
		Self {
			chars: chars.peekable(),
			line: 1,
			col: -1
		}
	}
	pub fn new_tok(&mut self, kind: TokenKind) -> Token {
		self.col += 1;
		Token::new(self.line, self.col,	kind)
	}
}

impl<Chars: Iterator<Item=char>> Iterator for Lexer<Chars> {
	
	type Item = Token;

	fn next(&mut self) -> Option<Token> {
		while let Some(x) = self.chars.next_if(|x| x.is_whitespace()) {
			if x == '\n' {
				self.line += 1;
			} else {
				self.col += 1;
			}
		}

		if let Some(x) = self.chars.next() {
			let mut text = String::new();
			text.push(x);
			match x {
				'(' => Some(self.new_tok(TokenKind::OpenParan)),
				')' => Some(self.new_tok(TokenKind::CloseParan)),
				'+' => Some(self.new_tok(TokenKind::Plus)),
				'-' => Some(self.new_tok(TokenKind::Minus)),
				_ => {
					if !x.is_alphanumeric() {
						todo!("Report unexpected token properly starts with '{}'", x);
					}

					if x.is_alphabetic() {
						while let Some(x) = self.chars.next_if(|x| x.is_alphanumeric()) {
							text.push(x);
						}
						match text.as_str() {
							"defun" => Some(self.new_tok(TokenKind::Keyword(KeywordKind::Defun))),
							_ => Some(self.new_tok(TokenKind::Ident(text)))
						}
					} else {
						while let Some(x) = self.chars.next_if(|x| x.is_numeric()) {
							text.push(x);
						}
						Some(self.new_tok(TokenKind::Int(text.parse::<i64>().unwrap())))
					}
				}
			}
		} else {
			None
		}
	}
}

