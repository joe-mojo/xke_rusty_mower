use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq)]
pub struct Lawn {
	pub width: usize,
	pub height: usize
}

fn parse_usize(maybe_line: Option<&str>, what: &str) -> Result<usize, String> {
	maybe_line.map(|wstr:&str| Ok(wstr))
		.unwrap_or_else(|| Err(format!("{} not found in {:?}", what, maybe_line)))
		.and_then(|wstr:&str|
			wstr.parse::<usize>().map_err(|err: ParseIntError| err.to_string())
		)
}

impl Lawn {

	pub fn parse(line: String) -> Result<Lawn, String> {
		let mut line_split = line.split(" ");
		let w = parse_usize(line_split.next(), "width");
		let h = parse_usize(line_split.next(), "height");

		match (w, h) {
			(Ok(wnum), _) if wnum == 0 => Err("Width must be greater 0".to_string()),
			(_, Ok(hnum)) if hnum == 0 => Err("Height must be greater 0".to_string()),
			(Ok(wnum), Ok(hnum)) => Ok(Lawn {width:wnum, height:hnum} ),
			(Err(e), _) => Err(format!("Invalid width for Lawn in \"{}\" because of {}", line,  e)),
			(_, Err(e)) => Err(format!("Invalid height for Lawn in \"{}\" because of {}", line,  e)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_should_parse_lawn_dimensions() {
		assert_eq!(Lawn::parse("345 567".to_string()), Ok(Lawn{width: 345, height: 567}));
		assert_eq!(Lawn::parse("5 12".to_string()), Ok(Lawn{width: 5, height: 12}));
		assert_eq!(Lawn::parse("2000 1".to_string()), Ok(Lawn{width: 2000, height: 1}));
	}

	#[test]
	fn parse_should_return_the_right_error() {
		assert_eq!(
			Lawn::parse("-1 3".to_string()),
			Err("Invalid width for Lawn in \"-1 3\" because of invalid digit found in string".to_string())
		);
		assert_eq!(
			Lawn::parse("ae 3".to_string()),
			Err("Invalid width for Lawn in \"ae 3\" because of invalid digit found in string".to_string())
		);
		assert_eq!(
			Lawn::parse("1 -3".to_string()),
			Err("Invalid height for Lawn in \"1 -3\" because of invalid digit found in string".to_string())
		);
		assert_eq!(Lawn::parse("0 10".to_string()), Err("Width must be greater 0".to_string()));
		assert_eq!(Lawn::parse("10 0".to_string()), Err("Height must be greater 0".to_string()));
		assert_eq!(Lawn::parse("10".to_string()), Err("Invalid height for Lawn in \"10\" because of height not found in None".to_string()));
	}
}
