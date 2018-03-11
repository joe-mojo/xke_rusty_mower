use std::vec::Vec;

#[derive(Debug)]
pub enum Command {
	G, D, A
}

impl Command {

	pub fn from_char(c: &char) -> Option<Command>{
		match *c {
			'G' => Some(Command::G),
			'D' => Some(Command::D),
			'A' => Some(Command::A),
			_ => None
		}
	}

	pub fn parse(line: String) -> Result<Vec<Command>, String> {
		let empty_vec:Vec<Command> = Vec::with_capacity(line.len());
		line.chars().map(|c| {
			Command::from_char(&c).ok_or(format!("Cannot parse commands: invalid char '{}' in line {}", c, line))
		}).fold(Ok(empty_vec), |acc: Result<Vec<Command>, String>, elt: Result<Command, String>| {
			acc.and_then(|mut v: Vec<Command>|{
				elt.map(|e: Command| {
					v.push(e);
					v
				})
			})
		})
	}
}