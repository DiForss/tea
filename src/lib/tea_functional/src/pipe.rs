use void::Void;
use either::Either;

/*
Pipes are a more general type of stream.
*/
pub trait Pipe<I> {
	type Out;
	type Final;

	fn pipe(&mut self, I) -> Either<Self::Out, Self::Final>;

	// combinators
	fn handle_with<S>(self, s: S) -> HandleWith<Self, S>
		where Self: Sized,
		      S: Pipe<Either<Self::Out, Self::Final>>
	{
		HandleWith(self, s)
	}

	// runners
	fn run<S>(mut self, mut s: S) -> S::Final
		where S: Sink<<Self as Pipe<()>>::Out>,
		      Self: Source + Sized
	{
		while let Either::Left(v) = self.pipe(()) {
			match s.pipe(v) {
				Either::Left(_) => (),
				Either::Right(last) => return last,
			}
		}
		unreachable!()
	}
}

/*
A Producer produces values
 */
pub trait Producer: Pipe<()> {}
impl<P: Pipe<()>> Producer for P {}

/*
A Source is a Producer that never ends
 */
pub trait Source: Producer<Final = Void> {}
impl<P: Producer<Final = Void>> Source for P {}

/*
A Consumer consumes values, and produces some type of output
 */
pub trait Consumer<I, A>: Pipe<I, Out = A> {}
impl<I, A, P: Pipe<I, Out = A>> Consumer<I, A> for P {}

/*
A Sink is a Consumer that produces Unit output
 */
pub trait Sink<I>: Consumer<I, ()> {}
impl<I, C: Consumer<I, ()>> Sink<I> for C {}

// Combinators
pub struct HandleWith<P, H>(P, H);

impl<I, P, H> Pipe<I> for HandleWith<P, H>
	where P: Pipe<I>,
	      H: Pipe<Either<P::Out, P::Final>>
{
	type Out = H::Out;
	type Final = H::Final;

	fn pipe(&mut self, input: I) -> Either<Self::Out, Self::Final> {
		self.1.pipe(self.0.pipe(input))
	}
}

/*
Unit is a Source of (). A common use is to drive other Producers
or Sources
 */
pub type Unit = ();
impl Pipe<()> for Unit {
	type Out = ();
	type Final = Void;

	fn pipe(&mut self, _: ()) -> Either<Self::Out, Self::Final> { Either::Left(()) }
}

/*
Null is a Sink that accepts values and discards them
 */
pub struct Null;
impl<I> Pipe<I> for Null {
	type Out = ();
	type Final = Void;

	fn pipe(&mut self, _: I) -> Either<Self::Out, Self::Final> { Either::Left(()) }
}

/*
StdinLn is a Producer of Strings that ends if there is an error in Stdin
 */
pub struct StdinLn;

use std::io::stdin;
use std::io::Error as IOError;
use std::io::BufRead;

impl Pipe<()> for StdinLn {
	type Out = String;
	type Final = Result<(), IOError>;

	fn pipe(&mut self, _: ()) -> Either<Self::Out, Self::Final> {
		let next = {
			let stdin = stdin();
			let lock = stdin.lock();
			let mut lines = lock.lines();
			let next = lines.next();

			next
		};

		match next {
			None => Either::Right(Ok(())),
			Some(Ok(v)) => Either::Left(v),
			Some(Err(e)) => Either::Right(Err(e)),
		}
	}
}
