use crate::repositories::get_client;
use postgres::{Error, Row};

/// Allows to manipulate single file content
pub trait IFileContentRepository {
	fn get_content(&self) -> Vec<String>;

	fn get_line(&self, index: u32) -> Option<String>;

	fn delete_line(&self, index: u32);

	fn insert_new_line(&self, after_index: u32, content: Option<String>);

	fn upsert_line(&self, index: u32, content: String);
}

pub fn new(file_id: i32) -> Box<dyn IFileContentRepository> {}

struct FileContentRepository {
	file_id: i32
}

impl IFileContentRepository for FileContentRepository {
	fn get_content(&self) -> Vec<String> {
		get_client()
			.query("SELECT content FROM files_lines WHERE file_id = $1 ORDER BY line_number", &[&self.file_id])
			.unwrap()
			.iter()
			.map(|row| { return row.get(0); })
			.collect()
	}

	fn get_line(&self, index: u32) -> Option<String> {
		return match get_client().query_one("SELECT content FROM files_lines WHERE file_id = $1 AND line_number = $2",
									 &[&self.file_id, &index],
		) {
			Ok(row) => Some(row.get(0)),
			Err(_) => None
		}
	}

	fn delete_line(&self, index: u32) {
		todo!()
	}

	fn insert_new_line(&self, after_index: u32, content: Option<String>) {
		todo!()
	}

	fn upsert_line(&self, index: u32, content: String) {
		todo!()
	}
}