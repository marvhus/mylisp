pub mod lexer;
use lexer::Lexer;

use std::{
	fs,
	vec::Vec,
	{io, io::Write},
};

fn read_file(path: &str) -> String {
	fs::read_to_string(path).expect("Unable to open file")
} 

fn repl() {
	println!("Press Ctrl + C to exit.");
	let mut text = String::new();
	loop {

		print!("=> ");
		io::stdout()
			.flush()
			.expect("Failed to flush stdout");
		io::stdin()
			.read_line(&mut text)
			.expect("Failed to read input");

		print!("[ ");
		for tok in Lexer::from_iter(text.chars()) {
			print!("{:?}, ", tok.kind);
		}
		println!("]");
		
		text.clear();
	}
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		repl();
		std::process::exit(0);
	}
	let path = &args[1];
	let text = read_file(path);
	for tok in Lexer::from_iter(text.chars()) {
		println!("{}", tok);
	}
}
