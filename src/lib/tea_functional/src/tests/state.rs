use super::super::state::State;
use super::super::state::StateFn;

#[test]
fn basic() {
	let st = StateFn(|num| ((), num + 1));
	let (v, s) = st.run(2);
	assert_eq!(v, ());
	assert_eq!(s, 3);
}

#[test]
fn basic_and_then() {
	let st = StateFn(|num| ((), num + 1)).and_then(|_| ((), 3));
	let (v, s) = st.run(4);
	assert_eq!(v, ());
	assert_eq!(s, 3);
}

#[test]
fn basic_get() {
	let st = StateFn(|num| ((), num + 1)).get();
	let (v, s) = st.run(4);
	assert_eq!(v, 5);
	assert_eq!(v, s);
}

#[test]
fn basic_put() {
	let st = StateFn(|num| ((), num + 1)).put(5);
	let (v, s) = st.run(0);
	assert_eq!(v, ());
	assert_eq!(s, 5);
}

#[test]
fn basic_update() {
	let st = StateFn(|num| ((), num + 1)).update(|num| ((), num + 1));
	let (v, s) = st.run(0);
	assert_eq!(v, ());
	assert_eq!(s, 2);
}
