use std::num::ParseIntError;

#[derive(Debug)]
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

	pub fn parse(line: &str) -> Result<Lawn, String> {
		let mut line_split = line.split(" ");
		let w = parse_usize(line_split.next(), "width");
		let h = parse_usize(line_split.next(), "height");

		match (w, h) {
			(Ok(wnum), Ok(hnum)) => Ok(Lawn {width:wnum, height:hnum} ),
			(Err(e), _) => Err(e),
			(_, Err(e)) => Err(e)
		}

	}
}

