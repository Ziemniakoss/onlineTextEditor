use std::fs::File;
use crate::models::{Project, User};
use crate::services::projects::GetError;
use crate::repositories::projects_files::IProjectsFilesRepository;

/// This service should be bound to one user and one project.
pub trait IProjectsFilesService {
	fn get(&self, id: i32) -> Option<File>;
	fn get_all(&self) -> Vec<File>;
	fn update(&self, file: File) -> Result<File, UpdateError>;
	fn create(&self, file: File) -> Result<File, CreationError>;
	fn delete(&self, file: File) -> Result<File, DeletionError>;
}

pub enum UpdateError {}

pub enum CreationError {}

pub enum DeletionError {}

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
			user,
			project: project.clone(),
			project_files_repository: crate::repositories::projects_files::new(project)
		})),
		Err(err) => match err {
			GetError::AccessDenied => Err(ServiceCreationError::UserDoesNotHaveAccessToProject),
			GetError::DoesNotExist => Err(ServiceCreationError::ProjectDoesNotExists)
		}
	};
}

struct ProjectsFilesService {
	user: User,
	project: Project,
	project_files_repository: Box<dyn IProjectsFilesRepository>
}

impl IProjectsFilesService for ProjectsFilesService {
	fn get(&self, id: i32) -> Option<File> {

		todo!()
	}

	fn get_all(&self) -> Vec<File> {
		todo!()
	}

	fn update(&self, file: File) -> Result<File, UpdateError> {
		todo!()
	}

	fn create(&self, file: File) -> Result<File, CreationError> {
		todo!()
	}

	fn delete(&self, file: File) -> Result<File, DeletionError> {
		todo!()
	}
}