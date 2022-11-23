use std::{fs, path::Path, io::Error as IoError};
use crate::buffer::Buffer;

#[derive(Debug, Default)]
pub struct Editor {
	buffers: Vec<Buffer>,
	active_idx: usize,
}

impl Editor {
	pub fn from_file(path: &Path) -> Result<Self, IoError> {
		let src = fs::read_to_string(path)?;
		Ok(Self {
			buffers: vec![Buffer::from_text(&src)],
			active_idx: 0,
		})
	}

	pub fn active_buffer(&mut self) -> Option<&mut Buffer> {
		self.buffers.get_mut(self.active_idx)
	}
}
