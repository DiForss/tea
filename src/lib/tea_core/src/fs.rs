use std::path::Path;
use std::fs::File;
use std::io::Error as IOError;
use std::io::Read;
use std::io::Write;

use buffer::Buffer;

pub fn load_file(path: &Path) -> Result<Buffer, IOError> {
	let mut buffer = Buffer {
		name: match path.file_name() {
			Some(name) => {
				match name.to_str() {
					Some(filename) => filename.to_owned(),
					_ => panic!("Filename contained illegal characters"),
				}
			}
			_ => {
				match path.as_os_str().to_str() {
					Some(filename) => filename.to_owned(),
					_ => panic!("Filename contained illegal characters"),
				}
			}
		},
		path: path.to_owned(),
		data: Vec::new(),
	};

	let mut file = File::open(&buffer.path)?;
	file.read_to_end(&mut buffer.data)?;

	Ok(buffer)
}

pub fn write_file(path: &Path, contents: &Vec<u8>) -> Result<(), IOError> {
	let mut file = File::open(path)?;
	file.write_all(contents)
}
