use crate::repositories::users::User;
use crate::models::Project;

pub fn new(user: User) {}

pub trait IProjectsService {
	fn create(&self, project: Project) -> Result<Project, ProjectServiceError>;
	fn update(&self, project: Project) -> Result<Project, ProjectServiceError>;
	fn delete(&self, project: Project) -> Result<(), ProjectServiceError>;
	fn grant_access(&self, project: Project, user: USer) -> Result<(), ProjectServiceError>;
	fn revoke_access(&self, project: Project, user: USer) -> Result<(), ProjectServiceError>;
}

pub enum ProjectServiceError {
	ProjectDoesNotExists,
	NoAccess
}


struct ProjectsService{
	user:User,

}

impl IProjectsService for ProjectsService{
	fn create(&self, project: Project) -> Result<Project, ProjectServiceError> {
		unimplemented!()
	}

	fn update(&self, project: Project) -> Result<Project, ProjectServiceError> {
		unimplemented!()
	}

	fn delete(&self, project: Project) -> Result<(), ProjectServiceError> {
		unimplemented!()
	}

	fn grant_access(&self, project: Project, user: _) -> Result<(), ProjectServiceError> {
		unimplemented!()
	}

	fn revoke_access(&self, project: Project, user: _) -> Result<(), ProjectServiceError> {
		unimplemented!()
	}
}
