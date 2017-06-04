use fs;

use std::path::Path;
use std::path::PathBuf;
use std::io::Error as IOError;

#[derive(Default)]
pub struct Buffer {
	pub name: String,
	pub path: PathBuf,
	pub data: Vec<u8>,
}

impl Buffer {
	pub fn new_from_path(path: &Path) -> Result<Buffer, IOError> { fs::load_file(path) }

	pub fn save(&self) -> Result<(), IOError> { fs::write_file(&self.path, &self.data) }
}
