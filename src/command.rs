use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub enum Command {
	//TODO 1 define command enum elements
	WRONG// <-- Remove this line once you have coded the real thing
}

impl Command {

	pub fn from_char(c: &char) -> Option<Command>{
		//TODO 2 Use pattern matching. Pay attention, your argument is a reference; you will need to dereference it to call `match`
		None // <-- Remove this line once you have coded the real thing
	}

	pub fn parse(line: String) -> Result<Vec<Command>, String> {
		let empty_vec:Vec<Command> = Vec::with_capacity(line.len());
		//TODO 3 This is where the party begins. Map chars of line to Command instance, then fold to a Result of Vec `Result<Vec<Command>, String>`
		//Hint: empty_vec is only the zero of the fold.
		//Hint: inside fold, there is somewhere a lambda that take a mut v: Vec<Command> as argument, because you will push elements into it.
		Ok(empty_vec) // <-- Remove this line once you have coded the real thing
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