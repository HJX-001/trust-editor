use std::{path::PathBuf, ops::Range, io::Error as IoError};

#[derive(Debug)]
pub struct Entry {
    range: Range<usize>,
    old_txt: String,
}

impl Entry {
    pub fn new(range: Range<usize>, old_txt: String) -> Self { Self { range, old_txt } }
}

#[derive(Debug, Default)]
pub struct History {
	pub undo: Vec<Entry>,
	pub redo: Vec<Entry>,
}

#[derive(Debug, Default)]
pub struct Buffer {
	pub text: String,
	pub offset: usize,
	pub path: Option<PathBuf>,
	pub modified: bool,
	pub history: History,
}

impl Buffer {
	pub fn from_text(text: &str) -> Self {
		Self {
			text: text.to_string(),
			offset: text.len(),
			..Default::default()
		}
	}

	pub fn set_path(&mut self, path: &PathBuf) -> Result<(), IoError>{
		self.path = Some(path.canonicalize()?);
		Ok(())
	}

	pub fn move_offset(&mut self, offset: usize) {
		self.offset = offset;
	}

	pub fn select(&mut self, start: usize) -> (usize, usize) {
		if start > self.offset {
			(self.offset, start)
		} else {
			(start, self.offset)
		}
	}

	fn edit_internal(&mut self, range: Range<usize>, text: &str) -> String {
		let old = self.text[range.clone()].to_string();
		self.text.replace_range(range.clone(), text);
		self.offset = self.offset + text.len() - range.len();
		self.modified = true;
		old
	}

	pub fn replace(&mut self, start_idx: usize, text: &str) {
		let (start, end) = self.select(start_idx);
		let old = self.edit_internal(start..end, text);
		self.history.undo.push(Entry::new(start..(start+text.len()), old));
		self.history.redo.clear();
	}

	pub fn undo(&mut self) {
		if let Some(last) = self.history.undo.pop() {
			let (start, end, old_txt) = (last.range.start, last.range.end, last.old_txt.as_str());
			let old = self.edit_internal(start..end, old_txt);
			self.history.redo.push(Entry::new(start..(start+old_txt.len()), old));
		}
	}

	pub fn redo(&mut self) {
		if let Some(last) = self.history.redo.pop() {
			let (start, end, old_txt) = (last.range.start, last.range.end, last.old_txt.as_str());
			let old = self.edit_internal(start..end, old_txt);
			self.history.undo.push(Entry::new(start..(start+old_txt.len()), old));
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn edit() {
		let mut a = Buffer::from_text("hello");
		a.move_offset(5);
		a.replace(5,"World");
		let d = Buffer::from_text("helloWorld");
		assert_eq!(a.text, d.text);

		a.replace(10, "");
		let c = Buffer::from_text("helloWorld");
		assert_eq!(a.text, c.text);
	}

	#[test]
	fn undo_redo() {
		let mut data = Buffer::from_text("");
	    for i in 10..20 {
	        data.replace(data.offset, i.to_string().as_str());
	    }

	    assert_eq!(data.text.as_str(), "10111213141516171819");

	    data.undo();
	    assert_eq!(data.text.as_str(), "101112131415161718");
	    data.undo();
	    assert_eq!(data.text.as_str(), "1011121314151617");
	    data.redo();
	    assert_eq!(data.text.as_str(), "101112131415161718");

	    data.replace(data.offset, 11.to_string().as_str());
	    assert_eq!(data.text.as_str(), "10111213141516171811");

	    data.redo();
	    assert_eq!(data.text.as_str(), "10111213141516171811");

	    data.undo();
	    assert_eq!(data.text.as_str(), "101112131415161718");
	    data.redo();
	    assert_eq!(data.text.as_str(), "10111213141516171811");
	    data.redo();
	    assert_eq!(data.text.as_str(), "10111213141516171811");
	}
}