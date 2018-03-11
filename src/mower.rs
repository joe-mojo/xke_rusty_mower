use regex::Regex;
use regex::Captures;
use direction::Direction;
use position::Position;
use command::Command;
use lawn::Lawn;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Mower {
	pub id: String,
	pub position: Position,
	pub direction: Direction,
	pub working_area: Lawn
}

impl Mower {
	fn capture_usize(idx: usize, captures: &Captures) -> Option<usize> {
		captures.get(idx).map(|m| m.as_str()).and_then(|num_str| num_str.parse::<usize>().ok())
	}

	fn capture_char(idx: usize, captures: &Captures) -> Option<char> {
		captures.get(idx).map(|m| m.as_str()).and_then(|char_str| char_str.chars().next())
	}

	pub fn parse(line: String, id: String, lawn: &Lawn) -> Result<Mower, String> {
		lazy_static! {
			static ref REGX: Regex = Regex::new(r"^(\d+) (\d+) ([NSEOW])$").unwrap();
		}

		REGX.captures(line.as_str()).and_then(|captures: Captures| {
			let maybe_x = Mower::capture_usize(1, &captures);
			let maybe_y = Mower::capture_usize(2, &captures);
			let maybe_dir = Mower::capture_char(3, &captures).and_then(|d| Direction::from_char(&d));
			maybe_x.and_then(|x| {
				maybe_y.and_then(|y| {
					maybe_dir.map(|dir|{
						Mower {
							id,
							position: Position { x, y},
							direction: dir,
							working_area: lawn.clone()
						}
					})
				})
			})
		}).ok_or(format!("Cannot parse Mower from {}", line))
	}

	fn next_position(&self) -> Position {
		let future_position = self.position.clone() + self.direction.go();
		if future_position.x < self.working_area.width && future_position.y < self.working_area.height {
			future_position
		} else {
			self.position.clone()
		}
	}

	fn apply(&mut self, command: &Command) -> () {
		match *command {
			Command::G => self.direction = self.direction.turn_left(),
			Command::D => self.direction = self.direction.turn_right(),
			Command::A => {
				self.position = self.next_position();
			}
		}
	}

	pub fn exec(&self, commands: Vec<Command>) -> Mower {
		let mut new_mower = self.clone();
		commands.iter().for_each(|cmd| new_mower.apply(cmd));
		new_mower
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_should_return_a_mower() {
		let lawn = Lawn{ width: 10, height:15};
		assert_eq!(
			Mower::parse("1 14 S".to_string(), "Mower One".to_string(), &lawn),
			Ok(Mower{id: "Mower One".to_string(), position: Position{x:1, y:14}, direction:Direction::South, working_area: lawn.clone()})
		);
		assert_eq!(
			Mower::parse("2 3 O".to_string(), "Mowmow".to_string(), &lawn),
			Ok(Mower{id: "Mowmow".to_string(), position: Position{x:2, y:3}, direction:Direction::West, working_area: lawn.clone()})
		);
		assert_eq!(
			Mower::parse("2 3 W".to_string(), "Mowmow".to_string(), &lawn),
			Ok(Mower{id: "Mowmow".to_string(), position: Position{x:2, y:3}, direction:Direction::West, working_area: lawn.clone()})
		);
		assert_eq!(
			Mower::parse("8 0 E".to_string(), "Mowmow".to_string(), &lawn),
			Ok(Mower{id: "Mowmow".to_string(), position: Position{x:8, y:0}, direction:Direction::East, working_area: lawn.clone()})
		);
		assert_eq!(
			Mower::parse("0 0 N".to_string(), "Chtimow".to_string(), &lawn),
			Ok(Mower{id: "Chtimow".to_string(), position: Position{x:0, y:0}, direction:Direction::North, working_area: lawn.clone()})
		);
	}

	#[test]
	fn parse_should_return_error_on_invalid_direction() {
		let lawn = Lawn{ width: 4, height:6};
		assert_eq!(
			Mower::parse("1 14 Z".to_string(), "Wrong direction".to_string(), &lawn),
			Err("Cannot parse Mower from 1 14 Z".to_string())
		);
	}

	#[test]
	fn parse_should_return_error_on_invalid_position() {
		let lawn = Lawn{ width: 4, height:6};
		assert_eq!(
			Mower::parse("-1 14 N".to_string(), "Wrong x".to_string(), &lawn),
			Err("Cannot parse Mower from -1 14 N".to_string())
		);
		assert_eq!(
			Mower::parse("1 ? N".to_string(), "Wrong y".to_string(), &lawn),
			Err("Cannot parse Mower from 1 ? N".to_string())
		);
	}

	#[test]
	fn exec_should_move_mower_to_the_right_location() {
		let lawn = Lawn{ width: 4, height:6};
		let commands = vec![Command::A, Command::D, Command::A];
		let init_mower = Mower {id:"M1".to_string(), position: Position{x:1, y:1}, direction:Direction::North, working_area: lawn.clone()};
		assert_eq!(
			init_mower.exec(commands),
			Mower {id:"M1".to_string(), position: Position{x:2, y:2}, direction: Direction::East, working_area: lawn.clone()}
		);

		let commands = vec![Command::A, Command::G, Command::A];
		let init_mower = Mower {id:"M2".to_string(), position: Position{x:2, y:2}, direction:Direction::South, working_area: lawn.clone()};
		assert_eq!(
			init_mower.exec(commands),
			Mower {id:"M2".to_string(), position: Position{x:3, y:1}, direction: Direction::East, working_area: lawn.clone()}
		);
	}

	#[test]
	fn exec_should_not_allow_mower_to_escape_lawn() {
		let lawn = Lawn{ width: 3, height:3};
		let commands = vec![
			Command::A, Command::D, Command::A, //trying to escape at south-west
			Command::D, Command::A, Command::A, Command::A, //to north limit
			Command::G, Command::A, //trying north-west
			Command::D, Command::D, Command::A, Command::A, Command::A, //to east limit
			Command::G, Command::A, //trying north-east
			Command::D, Command::D, Command::A, Command::A, Command::A, //to south limit
			Command::G, Command::A //trying south-east
		];
		let init_mower = Mower {id:"PrisonBreak".to_string(), position: Position{x:0, y:0}, direction:Direction::South, working_area: lawn.clone()};
		assert_eq!(
			init_mower.exec(commands),
			Mower {id:"PrisonBreak".to_string(), position: Position{x:2, y:0}, direction: Direction::East, working_area: lawn.clone()}
		);
	}

	#[test]
	fn apply_should_change_mower_direction(){
		//Given
		let lawn = Lawn{width:8, height:3};
		let mower_id = "Muted ;p".to_string();
		let mut mower = Mower{id:mower_id.clone(), position:Position{x:1,y:1}, direction:Direction::North, working_area: lawn.clone()};

		//When
		mower.apply(&Command::G);

		//Then
		assert_eq!(
			mower,
			Mower{id:mower_id.clone(), position:Position{x:1,y:1}, direction:Direction::West, working_area: lawn.clone()}
		);

		//Given
		let mut mower = Mower{id:mower_id.clone(), position:Position{x:1,y:1}, direction:Direction::North, working_area: lawn.clone()};

		//When
		mower.apply(&Command::D);

		//Then
		assert_eq!(
			mower,
			Mower{id:mower_id.clone(), position:Position{x:1,y:1}, direction:Direction::East, working_area: lawn.clone()}
		);
	}

	#[test]
	fn apply_should_change_mower_position() {
		//Given
		let lawn = Lawn{width:8, height:3};
		let mower_id = "Muted ;p".to_string();
		let mut mower = Mower{id:mower_id.clone(), position:Position{x:1,y:1}, direction:Direction::North, working_area: lawn.clone()};

		//When
		mower.apply(&Command::A);

		//Then
		assert_eq!(
			mower,
			Mower{id:mower_id.clone(), position:Position{x:1,y:2}, direction:Direction::North, working_area: lawn.clone()}
		);
	}

	#[test]
	fn next_position_should_return_the_position_forward() {
		let mower = Mower{id:"FWD".to_string(), position:Position{x:1,y:1}, direction:Direction::North, working_area: Lawn{width:3, height:3}};
		assert_eq!(mower.next_position(), Position{x:1, y:2});

		let mower = Mower{id:"FWD".to_string(), position:Position{x:1,y:1}, direction:Direction::East, working_area: Lawn{width:3, height:3}};
		assert_eq!(mower.next_position(), Position{x:2, y:1});

		let mower = Mower{id:"FWD".to_string(), position:Position{x:1,y:1}, direction:Direction::South, working_area: Lawn{width:3, height:3}};
		assert_eq!(mower.next_position(), Position{x:1, y:0});

		let mower = Mower{id:"FWD".to_string(), position:Position{x:1,y:1}, direction:Direction::West, working_area: Lawn{width:3, height:3}};
		assert_eq!(mower.next_position(), Position{x:0, y:1});
	}
}