use std::ops::Add;
use std::cmp::max;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
	pub x: usize,
	pub y: usize
}

#[derive(Debug, PartialEq)]
pub struct Move {
	pub dx: isize,
	pub dy: isize
}

impl Add<Move> for Position {
	type Output = Position;
	fn add(self, other: Move) -> Position {
		let mut pos = Position {x:0, y:0};
		if other.dx >= 0 {
			pos.x = self.x + (other.dx as usize);
		} else {
			pos.x = max((self.x as isize) - (-other.dx), 0) as usize;
		}
		if other.dy >= 0 {
			pos.y = self.y + (other.dy as usize);
		} else {
			pos.y = max((self.y as isize) - (-other.dy), 0) as usize;
		}
		pos
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn adding_move_gives_new_position() {
		let start = Position {x: 3, y:3};
		let move_east = Move {dx: 1, dy:0};
		assert_eq!(start.clone() + move_east, Position {x:4, y:3});

		let move_south = Move {dx: 0, dy:-1};
		assert_eq!(start.clone() + move_south, Position {x:3, y:2});

		let move_west = Move {dx: -1, dy:0};
		assert_eq!(start.clone() + move_west, Position {x:2, y:3});

		let move_north = Move {dx: 0, dy:1};
		assert_eq!(start.clone() + move_north, Position {x:3, y:4});
 	}
}