#![cfg_attr(feature="clippy", feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]

mod state;
mod fs;

use state::State;

fn run() {
	let state = State::initial_state();
	println!("Pouring tea...")
}

fn main() { run() }
