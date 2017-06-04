use tea_functional::void::Void;
use tea_functional::stream::Poll;
use std::fmt::Debug;

pub trait Handler<I, E>: Fn(I) -> Poll<Void, Result<(), E>> where E: Debug {}
