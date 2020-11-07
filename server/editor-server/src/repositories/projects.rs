use crate::repositories::{get_client};
use postgres::Row;
use log::{error};
use crate::models::{Project, User};

pub fn new() -> Box<dyn IProjectsRepository> {
	Box::new(ProjectRepository {})
}

pub trait IProjectsRepository {
	fn create(&self, project: Project) -> Result<Project, ProjectsUpdateError>;

	fn update(&self, project: Project) -> Result<Project, ProjectsUpdateError>;

	fn get(&self, id: i32) -> Option<Project>;

	fn get_all_shared_to(&self, user: &User) -> Vec<Project>;

	fn get_all_owned_by(&self, user: &User) -> Vec<Project>;

	fn has_access(&self, project: &Project, user: &User) -> bool;

	fn grant_access(&self, project: &Project, user: &User) -> Result<(), AccessManagementError>;

	fn revoke_access(&self, project: &Project, user: &User) -> Result<(), AccessManagementError>;
}

pub enum AccessManagementError {
	UserIsOwner,
	UserDoesNotExists,
}

pub enum ProjectsUpdateError {
	ProjectWithSameNameExists,
	IllegalName,
	ProjectDoesNotExist,
}

struct ProjectRepository {}

impl IProjectsRepository for ProjectRepository {
	fn create(&self, project: Project) -> Result<Project, ProjectsUpdateError> {
		let error_code_or_project_id: i32 = get_client().query_one(
			"SELECT * FROM create_project($1, $2, $3)",
			&[&project.name, &project.description, &project.owner.id],
		).unwrap().get(0);

		return match error_code_or_project_id {
			-1 => {
				error!("Tried to create project with non existing user");
				panic!("aaa");
			}
			-2 => {
				Err(ProjectsUpdateError::IllegalName)
			}
			-3 => {
				Err(ProjectsUpdateError::ProjectWithSameNameExists)
			}
			_ => Ok(Project {
				id: Some(error_code_or_project_id),
				name: project.name,
				description: project.description,
				owner: project.owner,
			})
		};
	}

	fn update(&self, project: Project) -> Result<Project, ProjectsUpdateError> {
		unimplemented!()
	}

	fn get(&self, id: i32) -> Option<Project> {
		return match get_client().query(
			"SELECT p.id, p.name, p.description, u.id, u.name\
		FROM projects p\
		JOIN users u on p.owner_id = u.id \
		WHERE id = $1", &[&id],
		) {
			Ok(rows) => {
				if rows.len() != 1 {
					return None;
				}
				Some(convert_to_project(rows.first().unwrap()))
			}
			Err(error) => {
				error!("Error occured while fetching project {}: {}", id, error);
				None
			}
		};
	}

	fn get_all_shared_to(&self, user: &User) -> Vec<Project> {
		return get_client().query(
			"SELECT p.id, p.name, p.description, u.id, u.name
			FROM projects_shared_for_users psfu
			JOIN projects p on p.id = psfu.project_id
			JOIN users u on u.id = p.owner_id
			WHERE psfu.user_id = $1", &[&user.id],
		).unwrap().iter()
			.map(|row| convert_to_project(row))
			.collect();
	}

	fn get_all_owned_by(&self, user: &User) -> Vec<Project> {
		return get_client().query(
			"SELECT p.id, p.name, p.description, u.id, u.name
			FROM projects p
			JOIN users u on u.id = p.owner_id
			WHERE owner_id = $1",
			&[&user.id],
		).unwrap().iter()
			.map(|row| convert_to_project(&row))
			.collect();
	}

	fn has_access(&self, project: &Project, user: &User) -> bool {
		return match get_client().query_one("SELECT * FROM has_access_to_project($1, $2)", &[&project.id, &user.id]) {
			Ok(row) => {
				row.get(0)
			}
			Err(error) => {
				error!("Error occured while checking access to project: {}", error);
				false
			}
		};
	}

	fn grant_access(&self, project: &Project, user: &User) -> Result<(), AccessManagementError> {
		return match get_client().query_one(
			"SELECT * FROM grant_access_to_project($1, $2)", &[&project.id, &user.id],
		).unwrap().get(0) {
			-1 => Err(AccessManagementError::UserIsOwner),
			_ => Ok(())
		};
	}

	fn revoke_access(&self, project: &Project, user: &User) -> Result<(), AccessManagementError> {
		return match get_client().query_one(
			"SELECT * FROM revoke_access_to_project($1, $2)", &[&project.id, &user.id],
		).unwrap().get(0) {
			-1 => Err(AccessManagementError::UserDoesNotExists),
			_ => Ok(())
		};
	}
}

fn convert_to_project(row: &Row) -> Project {
	let owner = User {
		id: row.get(3),
		name: row.get(4),
	};
	Project {
		id: Some(row.get(0)),
		name: row.get(1),
		description: row.get(2),
		owner,
	}
}