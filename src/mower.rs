use regex::Regex;
use regex::Captures;
use regex::Match;
use direction::Direction;
use position::Position;

#[derive(Debug)]
pub struct Mower {
	pub position: Position,
	pub direction: Direction
}

impl Mower {
	fn capture_usize(idx: usize, captures: &Captures) -> Option<usize> {
		captures.get(idx).map(|m| m.as_str()).and_then(|num_str| num_str.parse::<usize>().ok())
	}

	fn capture_char(idx: usize, captures: &Captures) -> Option<char> {
		captures.get(idx).map(|m| m.as_str()).and_then(|char_str| char_str.chars().next())
	}

	pub fn parse(line: &str) -> Result<Mower, String> {
		lazy_static! {
			static ref REGX: Regex = Regex::new(r"^(\d+) (\d+) ([NSEO])$").unwrap();
		}

		REGX.captures(line).and_then(|captures: Captures| {
			let maybe_x = Mower::capture_usize(1, &captures);
			let maybe_y = Mower::capture_usize(2, &captures);
			let maybe_dir = Mower::capture_char(3, &captures).and_then(|d| Direction::from_char(&d));
			maybe_x.and_then(|x| {
				maybe_y.and_then(|y| {
					maybe_dir.map(|dir|{
						Mower {
							position: Position { x, y},
							direction: dir
						}
					})
				})
			})
		}).ok_or(format!("Cannot parse Mower from {}", line))
	}
}
/*
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
*/