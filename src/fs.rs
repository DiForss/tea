use state::{Buffer, State};
use std::path::PathBuf;
use std::option::Option::*;

pub fn read_file(path: &PathBuf, mut st: State) -> State {
	match path.file_name() {
		Some(s) => {
			let name = s.to_os_string().to_string_lossy().into_owned();
			st.buffers.push(Buffer::new(name));
			st
		}
		None => st,
	}
}
