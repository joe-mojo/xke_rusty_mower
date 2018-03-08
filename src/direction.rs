use position::Move;

#[derive(Debug, Clone)]
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