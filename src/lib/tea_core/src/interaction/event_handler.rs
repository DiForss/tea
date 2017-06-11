use tea_functional::either::Either;
use tea_functional::pipe::Pipe;

use interaction::event::{Event, EventParseErr};

use std::io::Error as IOError;

pub struct EventHandler;
impl Pipe<Either<Vec<Result<Event, EventParseErr>>, Result<(), IOError>>> for EventHandler {
	type Out = ();
	type Final = Result<(), IOError>;

	fn pipe(&mut self,
	        v: Either<Vec<Result<Event, EventParseErr>>, Result<(), IOError>>)
	        -> Either<Self::Out, Self::Final> {
		match v {
			Either::Left(events) => Either::Left(println!("{:?}", events)),
			Either::Right(v) => Either::Right(v),
		}
	}
}
