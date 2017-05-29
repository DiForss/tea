use super::super::either::Either;
use super::super::void::Void;

#[test]
fn basic() {
	let mut s: Either<u8, &str> = Either::Left(8);
	if let Either::Left(v) = s {
		assert_eq!(v, 8)
	} else {
		assert!(false)
	};

	s = Either::Right("hello");
	if let Either::Right(v) = s {
		assert_eq!(v, "hello")
	} else {
		assert!(false)
	};
}

#[test]
fn flip() {
	let s: Either<u8, Void> = Either::Left(4);
	if let Either::Right(v) = s.flip() {
		assert_eq!(4, v)
	} else {
		assert!(false)
	};

	assert_eq!(s, s.flip().flip());
}

#[test]
fn mapping() {
	let s: Either<u8, usize> = Either::Left(4)
		.map_left(|v| v + 1)
		.map_right(|_: Void| "hello")
		.map(|v| v * 2, str::len);

	if let Either::Left(v) = s {
		assert_eq!(v, 10)
	} else {
		assert!(false)
	}

	let s: Either<(), usize> = Either::Right("hello")
		.map(|_: Void| (), str::len)
		.map_right(|l| l / 2)
		.flip()
		.flip();

	if let Either::Right(v) = s {
		assert_eq!(v, 2);
	} else {
		assert!(false)
	}
}
