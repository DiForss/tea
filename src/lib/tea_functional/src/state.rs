pub trait State<S> {
	type Val;

	fn run(self, prev: S) -> (Self::Val, S);

	fn and_then<U, F>(self, f: F) -> AndThen<Self, F>
		where F: FnOnce(Self::Val) -> (U, S),
		      Self: Sized
	{
		AndThen(self, f)
	}

	fn get(self) -> Get<Self>
		where Self: Sized
	{
		Get(self)
	}

	fn put(self, s: S) -> Put<S>
		where Self: Sized
	{
		Put(s)
	}

	fn update<U, F>(self, f: F) -> AndThen<Get<Self>, F>
		where F: FnOnce(S) -> (U, S),
		      S: Copy,
		      Self: Sized
	{
		self.get().and_then(f)
	}
}

pub struct StateFn<F>(pub F);

impl<S, V, F> State<S> for StateFn<F>
    where F: FnOnce(S) -> (V, S)
{
	type Val = V;

	fn run(self, prev: S) -> (Self::Val, S) { (self.0)(prev) }
}

pub struct AndThen<S, F>(S, F);

impl<U, S, T, F> State<T> for AndThen<S, F>
	where S: State<T>,
	      F: FnOnce(S::Val) -> (U, T)
{
	type Val = U;

	fn run(self, prev: T) -> (Self::Val, T) {
		let (v, _) = self.0.run(prev);
		(self.1)(v)
	}
}

pub struct Get<S>(S);

impl<S, T> State<T> for Get<S>
	where S: State<T>,
	      T: Copy
{
	type Val = T;

	fn run(self, prev: T) -> (Self::Val, T) {
		let (_, s) = self.0.run(prev);

		(s, s)
	}
}

pub struct Put<V>(V);

impl<V> State<V> for Put<V> {
	type Val = ();

	fn run(self, _: V) -> (Self::Val, V) { ((), self.0) }
}
