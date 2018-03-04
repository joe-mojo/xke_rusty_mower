
pub enum Direction {
	North,
	East,
	South,
	West
}

impl Direction {
	const VALUES : Array = [Direction::North, Direction::East, Direction::South, Direction::West];

	fn dx(&self) -> isize {
		match *self {
			Direction::North => 0,
			Direction::East => 1,
			Direction::South => 0,
			Direction::West => -1
		}
	}

	fn dy(&self) -> isize {
		match *self {
			Direction::North => 1,
			Direction::East => 0,
			Direction::South => -1,
			Direction::West => 0
		}
	}

	fn turnRight(&self) -> Direction {
		match *self {
			Direction::North => Direction::East,
			Direction::East => Direction::South,
			Direction::South => Direction::West,
			Direction::West => Direction::North
		}
	}

	fn turnLeft(&self) -> Direction {
		match *self {
			Direction::North => Direction::West,
			Direction::East => Direction::North,
			Direction::South => Direction::East,
			Direction::West => Direction::South
		}
	}
}