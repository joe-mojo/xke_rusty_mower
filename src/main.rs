use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::str::Split;
use std::vec::Vec;
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

//TODO tests => refactor main ? (testability)

fn next_line<'a>(split: &mut Split<'a, &'a str>) -> Option<&'a str> {
	split.skip_while(|line| line.is_empty()).next()
}

fn exec_instructions(contents: String) -> Result<Vec<Mower>, String> {
	let mut line_split = contents.split("\n");
	let working_area: Lawn = Lawn::parse(next_line(&mut line_split).expect("First line with lawn dimension not found")).unwrap();
	let mut final_states: Vec<Mower> = Vec::new();
	let mut id_counter: usize = 0;
	let mut mower_line = next_line(&mut line_split);
	let mut command_line = next_line(&mut line_split);

	loop {
		let tried_mower = Mower::parse(mower_line.expect("Mower line not found"), id_counter.to_string(), &working_area)
			.and_then(|mower|{
				println!("Mower setup: {:?}", &mower);
				Command::parse(command_line.expect("Command line not found"))
					.map(|commands|{
						mower.exec(commands)
					})
			});

		final_states.push(tried_mower.unwrap());

		mower_line = next_line(&mut line_split);
		if mower_line.is_none() {
			break;
		}
		command_line = next_line(&mut line_split);
		id_counter = id_counter + 1;
	}

	Ok(final_states)
}

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

	let execution_report: Result<Vec<Mower>, String> = exec_instructions(contents);

	println!("Final mower states: {:?}", execution_report);

}
