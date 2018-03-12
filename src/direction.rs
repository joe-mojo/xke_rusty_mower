use position::Move;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
	//TODO 1 define enum instances
	WRONG// <-- Remove this line once you have coded the real thing
}

impl Direction {

	pub fn from_char(c: &char) -> Option<Direction> {
		//TODO 2 Use pattern matching. Pay attention, your argument is a reference; you will need to dereference it to call `match`
		None// <-- Remove this line once you have coded the real thing
	}

	pub fn go(&self) -> Move {
		//TODO 3 Another pattern matching. Create the right Move depending on direction. Same problem with reference.
		//See Move in position.rs. Remember that (0,0) is  at south-west and (max, max) at North-East
		Move {dx:0, dy:0}// <-- Remove this line once you have coded the real thing
	}

	pub fn turn_right(&self) -> Direction {
		//TODO 4 Yet another pattern matching. What is the next direction after you turned right ?
		Direction::WRONG// <-- Remove this line once you have coded the real thing
	}

	pub fn turn_left(&self) -> Direction {
		//TODO 5 bla bla bla ... you know the song.
		Direction::WRONG
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
		assert_eq!(Some(Direction::West), Direction::from_char(&'O')); //You must understand French. Period.
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