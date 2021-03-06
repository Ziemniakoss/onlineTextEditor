use crate::models::{ User, ProjectFile};
use crate::services::projects::GetError;
use crate::repositories::projects_files::{IProjectsFilesRepository, ProjectFileUpdateError, ProjectFileCreationError};

/// This service should be bound to one user and one project.
pub trait IProjectsFilesService {
	fn get(&self, id: i32) -> Option<ProjectFile>;
	fn get_all(&self) -> Vec<ProjectFile>;
	fn update(&self, file: ProjectFile) -> Result<ProjectFile, UpdateError>;
	fn create(&self, file: ProjectFile) -> Result<ProjectFile, CreationError>;
	fn delete(&self, file: ProjectFile) -> Result<(), DeletionError>;
}

pub enum UpdateError {
	DuplicateName,
	IllegalName,
	FileDoesNotExists,
}

pub enum CreationError {
	IllegalName,
	DuplicateNames,
}

pub enum DeletionError {
	FileDoesNotExistInProject
}

pub enum ServiceCreationError {
	UserDoesNotExists,
	ProjectDoesNotExists,
	UserDoesNotHaveAccessToProject,
}

pub fn new(user_id: i32, project_id: i32) -> Result<Box<dyn IProjectsFilesService>, ServiceCreationError> {
	let user;
	match crate::repositories::users::get_user(user_id) {
		Some(u) => user = u,
		None => return Err(ServiceCreationError::UserDoesNotExists)
	}
	let project_service = crate::services::projects::new(User { id: user_id, name: user.name.clone() });
	return match project_service.get(project_id) {
		Ok(project) => Ok(Box::new(ProjectsFilesService {
			project_files_repository: crate::repositories::projects_files::new(project),
		})),
		Err(err) => match err {
			GetError::AccessDenied => Err(ServiceCreationError::UserDoesNotHaveAccessToProject),
			GetError::DoesNotExist => Err(ServiceCreationError::ProjectDoesNotExists)
		}
	};
}

struct ProjectsFilesService {
	project_files_repository: Box<dyn IProjectsFilesRepository>,
}

impl IProjectsFilesService for ProjectsFilesService {
	fn get(&self, id: i32) -> Option<ProjectFile> {
		self.project_files_repository.get(id)
	}

	fn get_all(&self) -> Vec<ProjectFile> {
		self.project_files_repository.get_all()
	}

	fn update(&self, file: ProjectFile) -> Result<ProjectFile, UpdateError> {
		return match self.project_files_repository.update(&file) {
			Ok(_) => Ok(file),
			Err(err) => match err {
				ProjectFileUpdateError::IllegalName => Err(UpdateError::IllegalName),
				ProjectFileUpdateError::DuplicateNames => Err(UpdateError::DuplicateName),
				ProjectFileUpdateError::FileDoesNotExist => Err(UpdateError::FileDoesNotExists)
			}
		};
	}

	fn create(&self, file: ProjectFile) -> Result<ProjectFile, CreationError> {
		return match self.project_files_repository.create(file) {
			Ok(file) => Ok(file),
			Err(err) => match err {
				ProjectFileCreationError::IllegalName => Err(CreationError::IllegalName),
				ProjectFileCreationError::DuplicateNames => Err(CreationError::DuplicateNames)
			}
		};
	}

	fn delete(&self, file: ProjectFile) -> Result<(), DeletionError> {
		return  if self.project_files_repository.delete(file) {
			Ok(())
		} else {
			Err(DeletionError::FileDoesNotExistInProject)
		}
	}
}