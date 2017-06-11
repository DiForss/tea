use std::io::{Error as IOError, Write, stdout};

use tea_functional::either::Either;
use tea_functional::pipe::{Pipe, StdinLn};
use interaction::event::{Event, EventParseErr, KBEvent, MouseEvent};

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
	last: Vec<Event>,
}

impl CliHandler {
	fn new() -> Self { Self { last: Vec::new() } }

	fn parse(&mut self, line: String) -> Vec<Event> {
		line.split_whitespace()
		    .map(|cluster| {
			let it = cluster.split('-').peekable();
			if let Some(v) = it.peek() {
				match *v {
					"Mouse" => Some(parse_mouse_event(it)),
					_ => Some(parse_keyboard_event(it)),
				}
			} else {
				None
			}
		})
		    .filter(Option::is_none)
		    .collect()
	}
}

fn parse_mouse_event<I: Iterator>(it: I) -> Result<MouseEvent, EventParseErr> {}

fn parse_keyboard_event<I: Iterator>(it: I) -> Result<KBEvent, EventParseErr> {}

impl Pipe<Either<String, Result<(), IOError>>> for CliHandler {
	type Out = Vec<Event>;
	type Final = Result<(), IOError>;

	fn pipe(&mut self, v: Either<String, Result<(), IOError>>) -> Either<Self::Out, Self::Final> {
		match v {
			Either::Left(s) => {
				if s == "" {
					Either::Left(self.last.clone())
				} else {
					Either::Left(self.parse(s))
				}
			}
			Either::Right(Ok(_)) => Either::Right(Ok(())),
			Either::Right(Err(e)) => Either::Right(Err(e)),
		}
	}
}
