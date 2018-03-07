use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
extern crate regex;
#[macro_use] extern crate lazy_static;
mod lawn;
mod mower;
mod position;
mod direction;
mod command;
use mower::Mower;
use lawn::Lawn;
use command::Command;

//parse file = parse mowerInstruction *
//parse mowerInstruction = parse position, parse orientation, parse commands
//parse commands = parse command *


fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("No file specified. Usage:\n\n\trusty_mower <file>\n\n\tfile: instruction file for mowers");
		process::exit(1);
	}
	println!("Input file: {}", args[1]);
	let mut in_file = File::open(&args[1]).expect("file not found");

	let mut contents = String::new();
	in_file.read_to_string(&mut contents).expect("Something went wrong reading the file");
	println!("With text:\n{}", contents);

	let mut line_split = contents.split("\n");
	let lawn = Lawn::parse(line_split.next().expect("First line with lawn dimension not found"));

	let mower = Mower::parse(line_split.next().expect("Mower line not found"));

	let cmdRes = Command::parse(line_split.next().expect("Command line not found"));

	let result = match (lawn, mower) {
		(Ok(l), Ok(m)) => {
			println!("{:?} {:?}", l, m);
			Ok(())
		},
		(Err(e), _) => Err(e),
		(_, Err(e)) => Err(e)
	};

	match result {
		Ok(_) => println!("Done."),
		Err(e) => println!("Error: {}", e)
	}
}
