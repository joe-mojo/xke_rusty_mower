use std::fmt;
use std::string::String;

pub struct Position {
	x: usize,
	y: usize
}


/*
pub fn parse(line: String) -> Result<Position> {
	let coord_tokens = line.split(" ");
	let token_count = coord_tokens.count();
	if token_count == 2 {
		let coords: (Result<usize, String>, Result<usize, String>) = (
			coord_tokens.next().map(|x:String| x.parse()),
			coord_tokens.next().map(|y: String| y.parse())
		);
		match *coords {
			(Ok(x), Ok(y)) => Ok(Position { x, y }),
			(Err(xerr), Ok(_)) => Err(format!("Cannot parse x in {}. {}", line, xerr)),
			(Ok(_), Err(yerr)) =>  Err(format!("Cannot parse y in {}. {}", line, yerr)),
			(Err(xerr), Err(yerr)) => Err(format!("Cannot parse coords in {}: {} {}", line, xerr, yerr))
		}
	} else {
		Err(format!("Wrong coord element count: {} (expected 2)", token_count))
	}
}*/
