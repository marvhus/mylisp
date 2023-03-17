pub mod lexer;
use lexer::Lexer;

use std::fs;
use std::vec::Vec;

fn read_file(path: &str) -> String {
	fs::read_to_string(path).expect("Unable to open file")
} 

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		todo!("Implement REPL. For now just do `mylisp <file>`");
	}
	let path = &args[1];
	let text = read_file(path);
	for tok in Lexer::from_iter(text.chars()) {
		println!("{}", tok);
	}
}
