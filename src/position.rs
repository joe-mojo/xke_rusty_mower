use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Position {
	pub x: usize,
	pub y: usize
}

pub struct Move {
	pub dx: isize,
	pub dy: isize
}

impl Add<Move> for Position {
	type Output = Position;
	fn add(self, other: Move) -> Position {
		let mut pos = Position {x:0, y:0};
		if other.dx > 0 {
			pos.x = self.x + (other.dx as usize);
		} else {
			pos.x = self.x - ((-other.dx) as usize);
		}
		if other.dy > 0 {
			pos.y = self.y + (other.dy as usize);
		} else {
			pos.y = self.y - ((-other.dy) as usize);
		}
		pos
	}
}