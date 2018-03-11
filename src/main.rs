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

fn next_line<'b>(split: &mut Split<'b, &'b str>) -> Option<&'b str> {
	split.skip_while(|line| line.is_empty()).next()
}

fn next_2_lines<'a>(split: &mut Split<'a, &'a str>) -> Result<Option<(String, String)>, String> {
	match (next_line(split), next_line(split)) {
		(Some(mower_line), Some(cmd_line)) => Ok(Some((mower_line.to_string(), cmd_line.to_string()))),
		(None, _) => Ok(None),
		(Some(mower_line), None) => Err(format!("Expected 2 lines, found only one: \"{}\"", mower_line))
	}
}

fn parse_mower_and_commands(lines: (String, String), mower_id: String, working_area: &Lawn) -> Result<(Mower, Vec<Command>), String> {
	match (Mower::parse(lines.0, mower_id, working_area), Command::parse(lines.1)) {
		(Ok(mower), Ok(commands)) => Ok((mower, commands)),
		(Err(mesg), _) => Err(mesg),
		(_, Err(mesg)) => Err(mesg)
	}
}

fn run_mower(mower_and_cmds: (Mower, Vec<Command>)) -> Mower {
	mower_and_cmds.0.exec(mower_and_cmds.1)
}

fn exec_instructions(contents: String) -> Result<Vec<Result<Mower, String>>, String> {
	let mut line_split = contents.split("\n");
	let mut id_counter: usize = 0;


	let report_res = next_line(&mut line_split).map(|line_str| line_str.to_string())
		.ok_or("First line with lawn dimension not found".to_string())
		.and_then(Lawn::parse)
		.map(|working_area: Lawn|{
			let mut exec_report: Vec<Result<Mower, String>> = Vec::new();
			loop {
				let input: Result<Option<(String, String)>, String> = next_2_lines(&mut line_split);
				match input {
					Ok(Some(lines)) => {
						let mower_result = parse_mower_and_commands(
							lines,
							format!("M{}", id_counter),
							&working_area
						).map(run_mower);
						exec_report.push(mower_result);
						id_counter = id_counter + 1;
					},
					Ok(None) => {
						break;
					},
					Err(mesg) => {
						exec_report.push(Err(mesg));
					}
				};
			};
			exec_report
	});

	report_res
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

	let execution_report = exec_instructions(contents);


	match execution_report {
		Ok(mower_results) => {
			mower_results.iter().for_each(|res| println!("{:?}", res))
		},
		Err(mesg) => println!("Could not mow: {}", mesg)
	}

}
