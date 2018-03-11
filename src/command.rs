use std::vec::Vec;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
	use command::Command;
	use rand::random;

	#[test]
	fn from_char_should_convert_valid_char_as_command() {
		let expected = Command::G;
		let actual = Command::from_char(&'G');
		assert!(actual.is_some(), "'G' was not parsed as Command");
		assert_eq!(actual.unwrap(), expected);

		let expected = Command::A;
		let actual = Command::from_char(&'A');
		assert!(actual.is_some(), "'A' was not parsed as Command");
		assert_eq!(actual.unwrap(), expected);

		let expected = Command::D;
		let actual = Command::from_char(&'D');
		assert!(actual.is_some(), "'D' was not parsed as Command");
		assert_eq!(actual.unwrap(), expected);
	}

	#[test]
	fn from_char_should_return_none_for_other_chars(){
		for _ in 0..1000 {
			let one_char = random::<char>();
			if one_char != 'A' && one_char != 'G' && one_char != 'D' {
				assert_eq!(Command::from_char(&one_char), None)
			}
		}
	}

	#[test]
	fn parse_ok_when_all_commands_are_valid() {
		let expected = Ok(vec![Command::G, Command::A, Command::D, Command::A, Command::A]);
		assert_eq!(Command::parse("GADAA".to_string()), expected)
	}

	#[test]
	fn parse_ko_if_at_least_one_char_is_invalid() {
		assert_eq!(
			Err("Cannot parse commands: invalid char 'z' in line GADzA".to_string()),
			Command::parse("GADzA".to_string())
		);
	}
}