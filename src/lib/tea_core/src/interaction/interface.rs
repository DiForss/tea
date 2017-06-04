use tea_functional::stream::Stream;
use std::fmt::Debug;

pub trait Interface<I, E>: Stream<Value = I, Final = Result<(), E>>
	where E: Debug
{
	fn display(&self) {}

	fn close(&self) {}
}
