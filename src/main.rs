pub mod lexer;
use lexer::Lexer;

use std::fs;

fn read_file(path: &str) -> String {
	fs::read_to_string(path).expect("Unable to open file")
} 

fn main() {
	let text = read_file("file.lisp");
	for tok in Lexer::from_iter(text.chars()) {
		println!("{}", tok);
	}
}
