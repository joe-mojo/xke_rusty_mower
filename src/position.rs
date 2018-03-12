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
		//TODO add missing lines to return the right Position depending on the Move. Next Position = Old Position + Move
		//TODO remember that (0,0) is  at south-west and (max, max) at North-East
		//Hint: Don't forget the boundaries
		//Hint: type usize is unsigned => saves some conditions
		//Hint: we are operating on usize (unsigned) and isize (signed). Some conversions needed to avoid panic...
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