use std::io::{Error as IOError, Write, stdout};
use std::collections::HashSet;

use tea_structures::functional::either::Either;
use tea_structures::pipe::{Pipe, StdinLn};
use interaction::event::{Event, EventParseErr, KBEvent, KBMods, MouseButton, MouseEvent, State};

pub struct CliIn;
impl Pipe<()> for CliIn {
	type Out = String;
	type Final = Result<(), IOError>;

	fn pipe(&mut self, _: ()) -> Either<Self::Out, Self::Final> {
		print!("tea> ");
		match stdout().flush() {
			Ok(_) => (),
			Err(e) => return Either::Right(Err(e)),
		};

		StdinLn.pipe(())
	}
}

pub struct CliHandler {
	last: Vec<Result<Event, EventParseErr>>,
}

impl CliHandler {
	pub fn new() -> Self { Self { last: Vec::new() } }

	fn parse(&mut self, line: String) -> Vec<Result<Event, EventParseErr>> {
		line.split_whitespace()
		    .map(|cluster| if cluster.contains("Mouse") {
			         parse_mouse_event(cluster).map(|v| Either::Left(v))
			        } else {
			         parse_keyboard_event(cluster).map(|v| Either::Right(v))
			        })
		    .collect()
	}
}

fn parse_mouse_event(ev_str: &str) -> Result<MouseEvent, EventParseErr> {
	let mut it = ev_str.split('-');
	// discard "Mouse"
	it.next();

	let state = match it.next() {
		Some(s) => {
			match s {
				"Up" => State::Up,
				"Down" => State::Down,
				_ => return Err(EventParseErr::ParseFail("Bad MouseType".to_string())),
			}
		}
		None => return Err(EventParseErr::ParseFail("Missing MouseType".to_string())),
	};

	let button = match it.next() {
		Some(s) => {
			match s {
				"Left" => MouseButton::Left,
				"Right" => MouseButton::Right,
				"Middle" => MouseButton::Middle,
				_ => {
					match s.parse() {
						Ok(v) => MouseButton::Other(v),
						Err(_) => {
							return Err(EventParseErr::ParseFail("Failed to parse number for mouse button".to_string()))
						}
					}
				}
			}
		}
		None => return Err(EventParseErr::ParseFail("Missing button".to_string())),
	};

	let x = match it.next() {
		Some(s) => {
			match s.parse() {
				Ok(v) => v,
				Err(_) => {
					return Err(EventParseErr::ParseFail("Failed to parse y-coordinate".to_string()))
				}
			}
		}
		None => return Err(EventParseErr::ParseFail("Missing x-coordinate".to_string())),
	};

	let y = match it.next() {
		Some(s) => {
			match s.parse() {
				Ok(v) => v,
				Err(_) => {
					return Err(EventParseErr::ParseFail("Failed to parse y-coordinate".to_string()))
				}
			}
		}
		None => return Err(EventParseErr::ParseFail("Missing y-coordinate".to_string())),
	};

	return Ok(MouseEvent {
	              state,
	              button,
	              x,
	              y,
	          });
}

fn parse_keyboard_event(ev_str: &str) -> Result<KBEvent, EventParseErr> {
	let mut it = ev_str.split('-');

	let mut mods = HashSet::new();
	let mut key = '0';
	let mut state = State::Down;
	while let Some(s) = it.next() {
		if s.to_lowercase() == s {
			key = match s.chars().nth(0) {
				Some(k) => k,
				None => return Err(EventParseErr::ParseFail("Failed to get char".to_string())),
			};
			break;
		} else {
			match s {
				"C" => {
					mods.insert(KBMods::Ctrl);
				}
				"A" => {
					mods.insert(KBMods::Alt);
				}
				"M" => {
					mods.insert(KBMods::Meta);
				}
				"S" => {
					mods.insert(KBMods::Shift);
				}
				"Down" => state = State::Down,
				"Up" => state = State::Up,
				_ => return Err(EventParseErr::ParseFail("Unknown modifier".to_string())),
			};
		}
	}

	return Ok(KBEvent { key, state, mods });
}

impl Pipe<Either<String, Result<(), IOError>>> for CliHandler {
	type Out = Vec<Result<Event, EventParseErr>>;
	type Final = Result<(), IOError>;

	fn pipe(&mut self, v: Either<String, Result<(), IOError>>) -> Either<Self::Out, Self::Final> {
		match v {
			Either::Left(s) => {
				if s == "q" {
					return Either::Right(Ok(()));
				} else if s != "" {
					self.last = self.parse(s);
				}

				Either::Left(self.last.clone())
			}
			Either::Right(Ok(_)) => Either::Right(Ok(())),
			Either::Right(Err(e)) => Either::Right(Err(e)),
		}
	}
}
