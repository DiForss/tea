use identity::identity;

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
pub enum Either<A, B> {
	Left(A),
	Right(B),
}

impl<A, B> Either<A, B> {
	pub fn map<T, U, FA, FB>(self, fa: FA, fb: FB) -> Either<T, U>
		where FA: FnOnce(A) -> T,
		      FB: FnOnce(B) -> U
	{
		match self {
			Either::Left(a) => Either::Left((fa)(a)),
			Either::Right(b) => Either::Right((fb)(b)),
		}
	}

	pub fn map_left<T, FA>(self, fa: FA) -> Either<T, B>
		where FA: FnOnce(A) -> T
	{
		self.map(fa, identity)
	}

	pub fn map_right<T, FB>(self, fb: FB) -> Either<A, T>
		where FB: FnOnce(B) -> T
	{
		self.map(identity, fb)
	}

	pub fn flip(self) -> Either<B, A> {
		match self {
			Either::Left(a) => Either::Right(a),
			Either::Right(b) => Either::Left(b),
		}
	}
}
