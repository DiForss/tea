pub trait Observer {
	type Value;
	type Error;

	fn on_event(self, event: Result<Option<Self::Value>, Self::Error>) -> Self where Self: Sized;
}

pub trait Observable {
	type Value;
	type Error;

	fn add_observer<L>(self, observer: L) -> Self
		where L: Observer<Value = Self::Value, Error = Self::Error>,
		      Self: Sized;
	fn poll(&self) -> Result<Option<Self::Value>, Self::Error>;

	// 	fn copy(&self) -> Self;
	fn map<F, U>(self, mapper: F) -> Map<Self, F, Self::Value, U, Self::Error>
		where Self: Sized,
		      F: FnMut(Self::Value) -> U
	{
		return;
	}

	// 	fn map_err<F, R>(self, mapper: F) -> MapErr;
}

pub struct MapErr {}

pub struct Map<O, F, V, U, E>
	where F: FnMut(V) -> U,
	      O: Observable<Value = V, Error = E>
{
	observers: Vec<Observer<Value = U, Error = E>>,
	inner_stream: O,
	mapper: F,
}

impl<O, F, V, U, E> Observable for Map<O, F, V, U, E>
	where F: FnMut(V) -> U,
	      O: Observable<Value = V, Error = E>
{
	type Value = U;
	type Error = E;

	fn add_observer<L>(self, observer: L) -> Self
		where L: Observer<Value = Self::Value, Error = Self::Error>
	{
		self.inner_stream.add_observer(observer);
		self
	}

	fn poll(&self) -> Result<Option<U>, Self::Error> {
		let result = self.inner_stream.poll();
		match result {
			Ok(o) => {
				match o {
					Some(v) => Result::Ok(Some((self.mapper)(v))),
					None => Result::Ok(None),
				}
			}
		}
	}
}
