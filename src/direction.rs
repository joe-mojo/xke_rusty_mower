use position::Move;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
	North,
	East,
	South,
	West
}

impl Direction {

	pub fn from_char(c: &char) -> Option<Direction> {
		match *c {
			'N' => Some(Direction::North),
			'E' => Some(Direction::East),
			'S' => Some(Direction::South),
			'O' => Some(Direction::West),
			'W' => Some(Direction::West),
			_ => None
		}
	}

	pub fn go(&self) -> Move {
		match *self {
			Direction::North => Move { dx: 0, dy: 1},
			Direction::East => Move { dx: 1, dy: 0},
			Direction::South => Move { dx: 0, dy: -1},
			Direction::West => Move { dx: -1, dy: 0}
		}
	}

	pub fn turn_right(&self) -> Direction {
		match *self {
			Direction::North => Direction::East,
			Direction::East => Direction::South,
			Direction::South => Direction::West,
			Direction::West => Direction::North
		}
	}

	pub fn turn_left(&self) -> Direction {
		match *self {
			Direction::North => Direction::West,
			Direction::East => Direction::North,
			Direction::South => Direction::East,
			Direction::West => Direction::South
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::random;

	#[test]
	fn from_char_should_convert_valid_char_as_direction(){
		assert_eq!(Some(Direction::North), Direction::from_char(&'N'));
		assert_eq!(Some(Direction::East), Direction::from_char(&'E'));
		assert_eq!(Some(Direction::South), Direction::from_char(&'S'));
		assert_eq!(Some(Direction::West), Direction::from_char(&'O'));
		assert_eq!(Some(Direction::West), Direction::from_char(&'W'));
	}

	#[test]
	fn from_char_should_return_none_for_other_chars() {
		for _ in 0..1000 {
			let one_char = random::<char>();
			if one_char != 'A' && one_char != 'G' && one_char != 'D' {
				assert_eq!(Direction::from_char(&one_char), None)
			}
		}
	}

	#[test]
	fn turn_left_test() {
		assert_eq!(Direction::North.turn_left(), Direction::West);
		assert_eq!(Direction::East.turn_left(), Direction::North);
		assert_eq!(Direction::South.turn_left(), Direction::East);
		assert_eq!(Direction::West.turn_left(), Direction::South);
	}

	#[test]
	fn turn_right_test() {
		assert_eq!(Direction::North.turn_right(), Direction::East);
		assert_eq!(Direction::East.turn_right(), Direction::South);
		assert_eq!(Direction::South.turn_right(), Direction::West);
		assert_eq!(Direction::West.turn_right(), Direction::North);
	}

	#[test]
	fn go_should_move_coords() {
		assert_eq!(Direction::North.go(), Move{dx: 0, dy:1});
		assert_eq!(Direction::East.go(), Move { dx: 1, dy: 0});
		assert_eq!(Direction::South.go(), Move { dx: 0, dy: -1});
		assert_eq!(Direction::West.go(), Move { dx: -1, dy: 0});
	}
}