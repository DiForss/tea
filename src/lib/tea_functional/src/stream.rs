use void::Void;

pub enum Poll<T, D> {
	Some(T),
	Done(D),
}

/*
A stream pro duces a number of Values before finally producing a
Final. Streams are driven from downstream.
*/
pub trait Stream {
	type Value;
	type Final;

	fn poll(&mut self) -> Poll<Self::Value, Self::Final>;

	fn flush(self) -> Flush<Self>
		where Self: Sized
	{
		Flush(self)
	}

	fn take(self, n: u64) -> Take<Self>
		where Self: Sized
	{
		Take(n, self)
	}

	fn take_clone(self, n: u64) -> TakeClone<Self>
		where Self: Sized
	{
		TakeClone(n, self)
	}

	fn take_values(self, n: u64) -> TakeValues<Self>
		where Self: Sized
	{
		TakeValues(n, self)
	}

	fn grab(self) -> Grab<Self>
		where Self: Sized
	{
		Grab(self)
	}

	fn grab_value(self) -> GrabValue<Self>
		where Self: Sized
	{
		GrabValue(self)
	}

	fn map<U, F>(self, f: F) -> Map<Self, U, F>
		where Self: Sized,
		      F: Fn(Self::Value) -> U
	{
		Map(f, self)
	}

	fn map_final<U, F>(self, f: F) -> MapFinal<Self, U, F>
		where Self: Sized,
		      F: Fn(Self::Final) -> U
	{
		MapFinal(f, self)
	}

	fn discard_final(self) -> DiscardFinal<Self>
		where Self: Sized
	{
		DiscardFinal(self)
	}

	fn then<U, F>(self, f: F) -> Then<Self, U, F>
		where Self: Sized,
		      F: Fn(Self::Value) -> Poll<U, Self::Final>
	{
		Then(f, self)
	}

	fn fold<U, F>(&mut self, init: U, f: F) -> U
		where Self: Sized,
		      F: Fn(Self::Value, U) -> U
	{
		let mut acc = init;
		while let Poll::Some(v) = self.poll() {
			acc = f(v, acc)
		}

		acc
	}

	fn run(&mut self) -> Self::Final
		where Self: Consumer
	{
		if let Poll::Done(d) = self.poll() {
			return d;
		}

		unreachable!()
	}

	fn flush_run(self) -> Self::Final
		where Self: Sized
	{
		self.flush().run()
	}
}

// A generator never produces a Final
pub trait Generator: Stream<Final = Void> {}
impl<T: Stream<Final = Void>> Generator for T {}

// A consumer never produces any Values
pub trait Consumer: Stream<Value = Void> {}
impl<T: Stream<Value = Void>> Consumer for T {}


// Forever produces a single Value forever
#[derive(Copy, Clone)]
pub struct Forever<T>(T);
impl<T> Stream for Forever<T>
    where T: Copy
{
	type Value = T;
	type Final = Void;

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> { Poll::Some(self.0) }
}

pub fn forever<T>(v: T) -> Forever<T> { Forever(v) }

// ForeverClone is like Forever but with a Cloneable thing
#[derive(Copy, Clone)]
pub struct ForeverClone<T>(T);
impl<T> Stream for ForeverClone<T>
    where T: Clone
{
	type Value = T;
	type Final = Void;

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> { Poll::Some(self.0.clone()) }
}

pub fn forever_clone<T>(v: T) -> ForeverClone<T> { ForeverClone(v) }

// Unit always produces ()
pub type Unit = Forever<()>;

// Flush polls the upstream until it returns a result, and produces no values
pub struct Flush<S>(S);
impl<S> Stream for Flush<S>
    where S: Stream
{
	type Value = Void;
	type Final = S::Final;

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> {
		loop {
			match self.0.poll() {
				Poll::Some(_) => (),
				Poll::Done(d) => return Poll::Done(d),
			}
		}
	}
}

// StdinLn is a stream that reads lines from stdin
use std::io::Error as IOError;
use std::io::stdin;
use std::io::BufRead;

#[derive(Copy, Clone)]
pub struct StdinLn;
impl Stream for StdinLn {
	type Value = String;
	type Final = Result<(), IOError>;

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> {
		let next = {
			let stdin = stdin();
			let lock = stdin.lock();
			let mut lines = lock.lines();

			lines.next()
		};

		match next {
			Some(Ok(ln)) => Poll::Some(ln),
			Some(Err(e)) => Poll::Done(Err(e)),
			None => Poll::Done(Ok(())),
		}
	}
}

// Take takes N values from some upstream before returning whatever was left
#[derive(Copy, Clone)]
pub struct Take<S>(u64, S);
impl<S> Stream for Take<S>
    where S: Stream + Copy
{
	type Value = S::Value;
	type Final = S;

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> {
		println!("{}", self.0);

		if self.0 == 0 {
			return Poll::Done(self.1);
		}

		match self.1.poll() {
			Poll::Some(v) => {
				self.0 = self.0 - 1;
				Poll::Some(v)
			}
			Poll::Done(_) => Poll::Done(self.1),
		}
	}
}

// TakeClone is like Take, but for cloneable streams
#[derive(Copy, Clone)]
pub struct TakeClone<S>(u64, S);
impl<S> Stream for TakeClone<S>
    where S: Stream + Clone
{
	type Value = S::Value;
	type Final = S;

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> {
		println!("{}", self.0);

		if self.0 == 0 {
			return Poll::Done(self.1.clone());
		}

		match self.1.poll() {
			Poll::Some(v) => {
				self.0 = self.0 - 1;
				Poll::Some(v)
			}
			Poll::Done(_) => Poll::Done(self.1.clone()),
		}
	}
}

// TakeValues is like Take, but doesn't return the leftovers
#[derive(Copy, Clone)]
pub struct TakeValues<S>(u64, S);
impl<S> Stream for TakeValues<S>
    where S: Stream
{
	type Value = S::Value;
	type Final = ();

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> {
		println!("{}", self.0);

		if self.0 == 0 {
			return Poll::Done(());
		}

		match self.1.poll() {
			Poll::Some(v) => {
				self.0 = self.0 - 1;
				Poll::Some(v)
			}
			Poll::Done(_) => Poll::Done(()),
		}
	}
}

// Stash creates a stream that contains exactly one value
#[derive(Copy, Clone)]
pub struct Stash<T>(Option<T>);
impl<T> Stream for Stash<T> {
	type Value = T;
	type Final = ();

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> {
		match self.0.take() {
			Some(v) => Poll::Some(v),
			None => Poll::Done(()),
		}
	}
}

pub fn stash<T>(t: T) -> Stash<T> { Stash(Some(t)) }

// Grab tries to take a single value out of the stream
#[derive(Copy, Clone)]
pub struct Grab<S>(S);
impl<S> Stream for Grab<S>
    where S: Stream + Copy
{
	type Value = Void;
	type Final = Option<(S::Value, S)>;

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> {
		match self.0.poll() {
			Poll::Some(v) => Poll::Done(Some((v, self.0))),
			Poll::Done(_) => Poll::Done(None),
		}
	}
}

// GrabValue is like Grab, but doesn't return the stream
#[derive(Copy, Clone)]
pub struct GrabValue<S>(S);
impl<S> Stream for GrabValue<S>
    where S: Stream
{
	type Value = Void;
	type Final = Option<(S::Value)>;

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> {
		match self.0.poll() {
			Poll::Some(v) => Poll::Done(Some(v)),
			Poll::Done(_) => Poll::Done(None),
		}
	}
}

// Map maps all values produced by a stream to a different type
#[derive(Copy, Clone)]
pub struct Map<S, U, F>(F, S)
	where S: Stream,
	      F: Fn(S::Value) -> U;

impl<S, U, F> Stream for Map<S, U, F>
	where S: Stream,
	      F: Fn(S::Value) -> U
{
	type Value = U;
	type Final = S::Final;

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> {
		match self.1.poll() {
			Poll::Some(v) => Poll::Some((self.0)(v)),
			Poll::Done(d) => Poll::Done(d),
		}
	}
}

// MapFinal maps the final value of a stream
#[derive(Copy, Clone)]
pub struct MapFinal<S, U, F>(F, S)
	where S: Stream,
	      F: Fn(S::Final) -> U;

impl<S, U, F> Stream for MapFinal<S, U, F>
	where S: Stream,
	      F: Fn(S::Final) -> U
{
	type Value = S::Value;
	type Final = U;

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> {
		match self.1.poll() {
			Poll::Some(v) => Poll::Some(v),
			Poll::Done(d) => Poll::Done((self.0)(d)),
		}
	}
}

// DiscardFinal discards the final value of a stream
#[derive(Copy, Clone)]
pub struct DiscardFinal<S>(S);
impl<S> Stream for DiscardFinal<S>
    where S: Stream
{
	type Value = S::Value;
	type Final = ();

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> {
		match self.0.poll() {
			Poll::Some(v) => Poll::Some(v),
			_ => Poll::Done(()),
		}
	}
}

// Then applies a function that may end the stream to the stream
#[derive(Copy, Clone)]
pub struct Then<S, U, F>(F, S)
	where S: Stream,
	      F: Fn(S::Value) -> Poll<U, S::Final>;

impl<S, U, F> Stream for Then<S, U, F>
	where S: Stream,
	      F: Fn(S::Value) -> Poll<U, S::Final>
{
	type Value = U;
	type Final = S::Final;

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> {
		match self.1.poll() {
			Poll::Done(d) => Poll::Done(d),
			Poll::Some(v) => (self.0)(v),
		}
	}
}

// IterStream makes a stream from an iterator
#[derive(Copy, Clone)]
pub struct IterStream<I>(I);
impl<I> Stream for IterStream<I>
    where I: Iterator
{
	type Value = I::Item;
	type Final = ();

	fn poll(&mut self) -> Poll<Self::Value, Self::Final> {
		match self.0.next() {
			Some(v) => Poll::Some(v),
			None => Poll::Done(()),
		}
	}
}

pub fn iter_stream<I>(iter: I) -> IterStream<I> { IterStream(iter) }
