#![feature(conservative_impl_trait)]

#![cfg_attr(feature="clippy", feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]

mod buffer;
mod state;
mod fs;
mod stream;

use state::State;

fn run() {
	let state = State::initial_state();
	let mut keep_going = true;

	while keep_going {
		// poll events
		println!("Polling events");

		// check buffers for updates
		println!("Checking buffers for updates");

		// update the state
		println!("Updating state");

		keep_going = false;
	}
}

fn main() { run() }
