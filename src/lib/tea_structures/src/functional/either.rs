#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Either<L, R> {
	Left(L),
	Right(R),
}
