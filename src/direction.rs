
#[derive(Debug)]
pub enum Direction {
	North,
	East,
	South,
	West
}

impl Direction {
	//const VALUES: [Direction; 4] = [Direction::North, Direction::East, Direction::South, Direction::West];
	//const LETTERS:[char; 4] = ['N', 'E', 'S', 'O'];

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

	pub fn dx(&self) -> isize {
		match *self {
			Direction::North => 0,
			Direction::East => 1,
			Direction::South => 0,
			Direction::West => -1
		}
	}

	pub fn dy(&self) -> isize {
		match *self {
			Direction::North => 1,
			Direction::East => 0,
			Direction::South => -1,
			Direction::West => 0
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