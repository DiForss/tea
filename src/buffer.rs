use state;
use state::State;

pub enum Type {
	Blessed(Type),
	Text,
}

pub type ID = u32;

#[derive(Debug)]
pub struct Buffer {
	pub name: String,
	pub file_contents: Vec<u8>,
	pub id: ID,
	pub buffer_type: Type,
}

impl Buffer {
	pub fn spawn_buffer_into(&mut state: State, name: String, buffer_type: Type) -> Buffer {
		let id = state.last_id + 1;

		let buffer = Buffer {
			name,
			file_contents: Vec::new(),
			id,
			buffer_type,
		};

		state.last_id = id;
		state.buffers.insert(id, buffer);

		buffer
	}
}
