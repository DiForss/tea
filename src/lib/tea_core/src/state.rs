use std::vec::Vec;
use std::collections::HashMap;

use buffer;
use buffer::Buffer;

#[derive(Debug)]
pub struct State {
	pub buffers: HashMap<buffer::ID, Buffer>,
	pub active_buffer: buffer::ID,
	pub last_id: buffer::ID,
}

impl State {
	pub fn initial_state() -> State {
		let state = State {};
		Buffer::spawn_buffer_into(state,
		                          "*Messages*",
		                          buffer::Type::Blessed(buffer::Type::Text));
		Buffer::spawn_buffer_into(state,
		                          "*Messages*",
		                          buffer::Type::Blessed(buffer::Type::Text));
	}
}
