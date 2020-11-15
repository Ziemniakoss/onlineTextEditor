use crate::models::{ProjectFile, Project};
use crate::repositories::get_client;
use log::{info, warn};

/// Operates CRUD operations on files. Each IProjectsFilesRepository
/// implementation must operate only on one project
pub trait IProjectsFilesRepository {
	fn create(&self, file: ProjectFile) -> Result<ProjectFile, ProjectFileCreationError>;

	fn update(&self, file: &ProjectFile) -> Result<(), ProjectFileUpdateError>;

	fn delete(&self, file: ProjectFile);
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
					.query_one("SELECT * FROM create_file($1, $2)", &[&file_id, &file.name])
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
		}
	}

	fn delete(&self, file: ProjectFile) {
		match file.id{
			Some(file_id) => {
				get_client().execute("DELETE FROM files WHERE id = $1", &[&file_id]);
			}
			None => {
				warn!("SSomeone tried to delete file without id");
			}
		}
	}
}
