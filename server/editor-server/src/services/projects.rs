use crate::repositories::users::User;
use crate::models::Project;
use crate::repositories::projects::IProjectsRepository;
use std::fs::read_to_string;

pub fn new(user: User) {}

pub trait IProjectsService {
	fn create(&self, name: String, description: String) -> Result<Project, SaveError>;
	fn update(&self, project: Project) -> Result<Project, SaveError>;
	fn delete(&self, project: Project) -> Result<(), DeleteError>;
	fn grant_access(&self, project: &Project, user: &USer) -> Result<(), AccessGrantingError>;
	fn revoke_access(&self, project: &Project, user: &USer) -> Result<(), AccessRevokingError>;
}

pub enum DeleteError {
	AccessDenied
}

pub enum AccessGrantingError {
	NotOwner
}

pub enum AccessRevokingError {
	IsOwner
}

pub enum SaveError {
	InvalidName,
	ProjectDoesNotExists,
	AccessException,
}

struct ProjectsService {
	user: User,
	projects_repository: Box<dyn IProjectsRepository>,
}

impl IProjectsService for ProjectsService {
	fn create(&self, project: Project) -> Result<Project, SaveError> {
		match self.projects_repository.create(project)){}
	}

	fn update(&self, project: Project) -> Result<Project, SaveError> {
		if project.owner != self.user {
			return Err(SaveError::AccessException);
		}
		unimplemented!()
	}

	fn delete(&self, project: Project) -> Result<(), DeleteError> {
		unimplemented!()
	}

	fn grant_access(&self, project: &Project, user: &User) -> Result<(), AccessGrantingError> {
		unimplemented!()
	}

	fn revoke_access(&self, project: &Project, user: &User) -> Result<(), AccessRevokingError> {
		unimplemented!()
	}
}

