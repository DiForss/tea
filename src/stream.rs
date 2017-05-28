use std::marker::PhantomData;
use std::iter::Iterator;

#[derive(Copy, Clone)]
pub enum Void {}

#[derive(Copy, Clone)]
pub enum Event<T, E> {
	Done,
	None,
	Val(T),
	Err(E),
}

pub trait Stream {
	type Out;
	type Err;

	fn poll(&mut self) -> Event<Self::Out, Self::Err>;

	fn pipe<C: Consumer<In = Self::Out, Err = Self::Err>>(self, c: C) -> Pipe<Self, C>
		where Self: Sized
	{
		Pipe(self, c)
	}

	fn pipe_fn<T, F: Fn(T)>(self, f: F) -> Pipe<Self, FnConsumer<T, F>>
		where Self: Stream<Out = T, Err = Void> + Sized
	{
		Pipe(self, FnConsumer::new(f))
	}

	fn flush(&mut self) {
		loop {
			match self.poll() {
				Event::None | Event::Err(_) | Event::Val(_) => (),
				Event::Done => break,
			}
		}
	}

	fn by_ref(&mut self) -> &mut Self { self }
}

impl<A, I> Stream for I
    where I: Iterator<Item = A>
{
	type Out = A;
	type Err = Void;

	fn poll(&mut self) -> Event<Self::Out, Self::Err> {
		match self.next() {
			Some(v) => Event::Val(v),
			None => Event::Done,
		}
	}
}

pub trait Consumer {
	type In;
	type Err;

	fn consume(&mut self, Event<Self::In, Self::Err>);
}

pub struct FnConsumer<T, F: Fn(T)>(PhantomData<T>, F);

impl<T, F: Fn(T)> FnConsumer<T, F> {
	pub fn new(f: F) -> Self { FnConsumer(PhantomData::default(), f) }
}

impl<T, F: Fn(T)> Consumer for FnConsumer<T, F> {
	type In = T;
	type Err = Void;

	fn consume(&mut self, e: Event<Self::In, Self::Err>) {
		if let Event::Val(v) = e {
			(self.1)(v)
		}
	}
}

pub struct Pipe<S, C>(S, C);

impl<S, C> Stream for Pipe<S, C>
	where S: Stream<Out = C::In, Err = C::Err>,
	      C: Consumer,
	      C::In: Copy,
	      C::Err: Copy
{
	type Out = S::Out;
	type Err = S::Err;

	fn poll(&mut self) -> Event<Self::Out, Self::Err> {
		let ev = self.0.poll();
		self.1.consume(ev);

		ev
	}
}

impl<S, C> Consumer for Pipe<S, C>
	where S: Stream<Out = C::In, Err = C::Err>,
	      C: Consumer
{
	type In = C::In;
	type Err = C::Err;

	fn consume(&mut self, e: Event<Self::In, Self::Err>) { self.1.consume(e); }
}
