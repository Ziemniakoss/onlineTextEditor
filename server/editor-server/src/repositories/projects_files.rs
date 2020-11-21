use crate::models::{ProjectFile, Project};
use crate::repositories::get_client;
use log::{info, warn, error};
use postgres::{Row};

/// Operates CRUD operations on files. Each IProjectsFilesRepository
/// implementation must operate only on one project
pub trait IProjectsFilesRepository {
	fn create(&self, file: ProjectFile) -> Result<ProjectFile, ProjectFileCreationError>;

	fn update(&self, file: &ProjectFile) -> Result<(), ProjectFileUpdateError>;

	fn delete(&self, file: ProjectFile);

	fn get(&self, file_id: i32) -> Option<ProjectFile>;

	fn get_all(&self) -> Vec<ProjectFile>;
}

pub enum ProjectFileCreationError {
	IllegalName,
	DuplicateNames,
}

pub enum ProjectFileUpdateError {
	IllegalName,
	DuplicateNames,
	FileDoesNotExist,
}

pub fn new(project: Project) -> Box<dyn IProjectsFilesRepository> {
	Box::new(ProjectFileRepository { project })
}

struct ProjectFileRepository {
	project: Project
}

impl IProjectsFilesRepository for ProjectFileRepository {
	fn create(&self, mut file: ProjectFile) -> Result<ProjectFile, ProjectFileCreationError> {
		let result_code: i32 = get_client()
			.query_one("SELECT * FROM create_file($1, $2)", &[&file.name, &self.project.id])
			.unwrap()
			.get(0);

		return if result_code == -1 {
			Err(ProjectFileCreationError::IllegalName)
		} else if result_code == -2 {
			Err(ProjectFileCreationError::DuplicateNames)
		} else {
			file.id = Some(result_code);
			Ok(file)
		};
	}

	fn update(&self, file: &ProjectFile) -> Result<(), ProjectFileUpdateError> {
		return match file.id {
			Some(file_id) => {
				let result_code: i32 = get_client()
					.query_one("SELECT * FROM update_file($1, $2)", &[&file_id, &file.name])
					.unwrap()
					.get(0);
				match result_code {
					-1 => Err(ProjectFileUpdateError::IllegalName),
					-2 => Err(ProjectFileUpdateError::FileDoesNotExist),
					-3 => Err(ProjectFileUpdateError::DuplicateNames),
					_ => {
						info!("Updated file {}", file_id);
						Ok(())
					}
				}
			}
			None => Err(ProjectFileUpdateError::FileDoesNotExist)
		};
	}

	fn delete(&self, file: ProjectFile) {
		match file.id {
			Some(file_id) => {
				let _ = get_client().execute("DELETE FROM files WHERE id = $1", &[&file_id]);
			}
			None => {
				warn!("SSomeone tried to delete file without id");
			}
		}
	}

	fn get(&self, file_id: i32) -> Option<ProjectFile> {
		return match get_client()
			.query_one("SELECT id, name FROM files WHERE project_id = $1 AND id = $2",
				   &[&self.project.id, &file_id]){
			Ok(row) => Some(self.convert_to_project(&row)),
			Err(err) => {
				error!("Error happened when trying to get file in project: {}",err);
				None
			}
		}
	}

	fn get_all(&self) -> Vec<ProjectFile> {
		 get_client().query("SELECT id, name FROM files WHERE project_id = $1", &[&self.project.id])
			.unwrap()
			.iter()
			.map(|row| self.convert_to_project(row))
			.collect()
	}
}
impl ProjectFileRepository {
	fn convert_to_project(&self,row: &Row) ->ProjectFile{
		ProjectFile{
			id: row.get(0),
			name: row.get(1),
			project_id: self.project.id.clone().unwrap()
		}
	}
}