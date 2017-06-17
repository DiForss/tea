use tea_functional::either::Either;
use tea_functional::pipe::Pipe;

use interaction::event::{Event, EventParseErr};

pub trait InputParser<I, E>
	: Pipe<Either<I, Result<(), E>>, Out = Vec<Result<Event, EventParseErr>>, Final = Result<(), E>>
    {
}
impl<I, E, P: Pipe<Either<I, Result<(), E>>,
									 Out = Vec<Result<Event, EventParseErr>>,
									 Final = Result<(), E>>> InputParser<I, E> for P {}
