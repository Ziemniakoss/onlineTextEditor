use crate::models::{Project, User};
use crate::repositories::projects::{IProjectsRepository, ProjectsUpdateError, AccessManagementError};
use log::{error};

pub fn new(user: User) -> Box<dyn IProjectsService> {
	Box::new(ProjectsService {
		user,
		projects_repository: crate::repositories::projects::new(),
	})
}

pub trait IProjectsService {
	fn create(&self, name: String, description: String) -> Result<Project, SaveError>;
	fn update(&self, project: Project) -> Result<Project, SaveError>;
	fn delete(&self, project: Project) -> Result<(), DeleteError>;
	fn get_owned_projects(&self) -> Vec<Project>;
	fn get_projects_shared_to_user(&self) -> Vec<Project>;
	fn get(&self, id: i32) -> Result<Project, GetError>;
	fn grant_access(&self, project: &Project, user: &User) -> Result<(), AccessGrantingError>;
	fn revoke_access(&self, project: &Project, user: &User) -> Result<(), AccessRevokingError>;
}

pub enum GetError {
	AccessDenied,
	DoesNotExist,
}

pub enum DeleteError {
	AccessDenied
}

pub enum AccessGrantingError {
	NotOwner
}

pub enum AccessRevokingError {
	IsOwner,
	UserIsNotOwner,
	UserDoesNotExists,
}

pub enum SaveError {
	InvalidName,
	ProjectDoesNotExists,
	AccessException,
	ProjectWithSameNaeAlreadyExists,
}

struct ProjectsService {
	user: User,
	projects_repository: Box<dyn IProjectsRepository>,
}

impl IProjectsService for ProjectsService {
	fn create(&self, name: String, description: String) -> Result<Project, SaveError> {
		let project = Project::new(name, description, User { id: self.user.id, name: self.user.name.clone() });
		return match self.projects_repository.create(project) {
			Ok(project) => {
				Ok(project)
			}
			Err(error) => {
				match error {
					ProjectsUpdateError::ProjectWithSameNameExists => Err(SaveError::ProjectWithSameNaeAlreadyExists),
					ProjectsUpdateError::IllegalName => Err(SaveError::InvalidName),
					_ => {
						panic!("This, this this this yes i dunno")
					}
				}
			}
		};
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

	fn get_owned_projects(&self) -> Vec<Project> {
		self.projects_repository.get_all_owned_by(&self.user)
	}

	fn get_projects_shared_to_user(&self) -> Vec<Project> {
		self.projects_repository.get_all_shared_to(&self.user)
	}

	fn get(&self, id: i32) -> Result<Project, GetError> {
		return match self.projects_repository.get(id) {
			None => Err(GetError::DoesNotExist),
			Some(project) => {
				if self.projects_repository.has_access(&project, &self.user) {
					Ok(project)
				} else {
					Err(GetError::AccessDenied)
				}
			}
		};
	}

	fn grant_access(&self, project: &Project, user: &User) -> Result<(), AccessGrantingError> {
		if project.owner != self.user {
			return Err(AccessGrantingError::NotOwner);
		}
		return match self.projects_repository.grant_access(project, user) {
			Ok(_) => Ok(()),
			Err(_) => {
				error!("Hmm this is bad: this error should not possible occur because it was handled eariler");
				Err(AccessGrantingError::NotOwner)//again, this never happens, will fix if I will have time for this TODO
			}
		};
	}

	fn revoke_access(&self, project: &Project, user: &User) -> Result<(), AccessRevokingError> {
		if project.owner != self.user {
			return Err(AccessRevokingError::UserIsNotOwner);
		}
		if user == &self.user {
			return Err(AccessRevokingError::IsOwner);
		}

		return match self.projects_repository.revoke_access(project, user) {
			Ok(_) => Ok(()),
			Err(error) => {
				match error {
					AccessManagementError::UserDoesNotExists => Err(AccessRevokingError::UserDoesNotExists),
					AccessManagementError::UserIsOwner => {
						error!("This error should not occur because this case was handled earlier");
						Err(AccessRevokingError::IsOwner)
					}
				}
			}
		};
	}
}

