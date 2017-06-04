use super::super::stream::{Stream, forever, iter_stream, stash};
use std::ops::Add;

#[test]
fn test_stash_grab() {
	let (v, _) = stash(1).grab().flush().run().unwrap();
	assert_eq!(1, v);

	let (v, _) = stash(2)
		.grab()
		.map_final(|o| match o {
		               Some((i, v)) => Some((i * 2, v)),
		               None => None,
		           })
		.flush()
		.run()
		.unwrap();
	assert_eq!(4, v);

	let (v, _) = stash(4).map(&move |i| i * 2).grab().run().unwrap();
	assert_eq!(v, 8);
}

#[test]
fn area51() {
	assert_eq!(6, iter_stream([1, 2, 3].iter()).fold(0, Add::add));

	assert_eq!(18,
	           iter_stream(["hello ", "world ", "today!"].iter()).fold(0, |s, acc| {
		acc + s.len()
	}));

	let (v, _) = stash(iter_stream([1, 2, 3].iter())
	                       .take_values(1)
	                       .fold(0, Add::add))
		.grab()
		.flush()
		.run()
		.unwrap();
	assert_eq!(v, 1);
}

#[test]
fn test_forever_take() {
	assert_eq!(10, forever(10).take(3).fold(0, Add::add));
}
