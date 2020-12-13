use crate::repositories::get_client;
use log::{info, warn, error};

/// Allows to manipulate single file content
pub trait IFileContentRepository {
    fn get_content(&self) -> Vec<String>;

    fn get_line(&self, index: u32) -> Option<String>;

    fn delete_line(&self, index: u32);

    fn insert_new_line(&self, index: u32, content: Option<String>);

    fn update(&self, index: u32, content: String);

    fn get_lines(&self, from_inclusive: u32, to_inclusive: u32) -> Vec<String>;
}

pub fn new(file_id: i32) -> Box<dyn IFileContentRepository> {
    Box::new(FileContentRepository { file_id })
}

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
        return match get_client()
            .query_one("SELECT content FROM files_lines WHERE file_id = $1 AND line_number = $2",
                                            &[&self.file_id, &(index as i32)],
        ) {
            Ok(row) => Some(row.get(0)),
            Err(err) => {
                error!("Error while retriving line {}: {}", index, err);
                None}
        };
    }

    fn delete_line(&self, index: u32) {
        match get_client()
            .execute("DELETE FROM files_lines WHERE file_id = $1 AND line_number = $2", &[&self.file_id, &index]){
            Ok(_) => info!("Deleted line {} in file {}", index, self.file_id ),
            Err(err) => {
                error!("Error occurred while trying to delete line {} in file {}: {}", index, self.file_id, err);
            }
        }
    }

    fn insert_new_line(&self, index: u32, content: Option<String>) {
        let result_code: i32 = get_client()
            .query_one("SELECT * FROM insert_line_in_file($1, $2, $3)", &[&self.file_id, &(index as i32), &content])
            .unwrap()
            .get(0);
        match result_code {
            -1 => warn!("Someone tried to insert text in negative line"),
            -2 => error!("Someone tried to insert line in nonexisting file {}", self.file_id),
            _ => info!("Inserted new line in file {} on index {} with content \"{}\"", self.file_id, index, content.unwrap_or("".to_owned()))
        }
    }

    fn update(&self, index: u32, content: String) {
        match get_client()
            .execute("UPDATE files_lines SET content = $1 WHERE file_id = $2 AND line_number = $3", &[&content, &self.file_id, &(index as i32)]){
            Ok(_) => info!("Updated line {} in file {}", index, self.file_id),
            Err(err) => error!("Failed to update line {} in line {}: {}", index, self.file_id, err)
        }
    }

    fn get_lines(&self, from_inclusive: u32, to_inclusive: u32) -> Vec<String> {
        get_client()
            .query("SELECT content FROM files_lines WHERE file_id = $1 AND line_number >= $2 AND line_number <= $3 ORDER BY line_number",
                   &[&self.file_id, &from_inclusive, &to_inclusive])
            .unwrap()
            .iter()
            .map(|row| {return row.get(0)})
            .collect()
    }


}