use std::vec::Vec;

#[derive(Debug)]
pub struct Buffer {
	pub name: String,
	pub file_contents: Vec<u8>,
}

impl Buffer {
	pub fn new(name: String) -> Buffer {
		Buffer {
			name,
			file_contents: Vec::new(),
		}
	}
}

#[derive(Debug)]
pub struct State {
	pub buffers: Vec<Buffer>,
}

impl State {
	pub fn initial_state() -> State {
		let mut default_buffers = Vec::new();
		default_buffers.push(Buffer::new("*Messages*".to_owned()));
		default_buffers.push(Buffer::new("*scratch*".to_owned()));

		State { buffers: default_buffers }
	}
}
