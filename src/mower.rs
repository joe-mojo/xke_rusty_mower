use regex::Regex;
use regex::Captures;
use direction::Direction;
use position::Position;
use command::Command;
use lawn::Lawn;
use std::vec::Vec;

#[derive(Debug, Clone)]
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
			static ref REGX: Regex = Regex::new(r"^(\d+) (\d+) ([NSEO])$").unwrap();
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
