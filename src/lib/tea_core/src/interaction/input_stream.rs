use tea_functional::pipe::Pipe;
use std::fmt::Debug;

pub trait InputStream<V, E>: Pipe<(), Out = V, Final = Result<(), E>>
    where E: Debug
{
}

impl<V, E, P: Pipe<(), Out = V, Final = Result<(), E>>> InputStream<V, E> for P where E: Debug {}
