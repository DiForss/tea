#![feature(conservative_impl_trait)]

#![cfg_attr(feature="clippy", feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]

mod stream;

use stream::*;

fn run() {
	// testing
	let v = vec![10, 10, 10];
	let mut it = v.into_iter();
	{
		let i_ref = Iterator::by_ref(&mut it);
		i_ref.take(1)
		     .pipe_fn(|i| { (0..i / 10).pipe_fn(|i| println!("{}", i)).flush(); })
		     .pipe_fn(|i| println!("{}", i * 2))
		     .flush();
	}

	it.pipe_fn(|_| println!("hello")).flush();
}

fn main() { run() }
