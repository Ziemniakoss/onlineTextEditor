use crate::repositories::{get_client, DatabaseError};
use postgres::Row;
use log::{error, info};
use crate::models::{Project, User};
use futures::future::err;

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

	fn grant_access(&self, project: &Project, user: &User);

	fn revoke_access(&self, project: &Project, user: &User) -> Result<(), RevokingAccessError>;
}

pub enum AccessManagementError {
	UserIsOwner,
	UserDoesNotExists,
}

pub enum ProjectsUpdateError {
	ProjectWithSameNameExists,
	IllegalName,
	ProjectDoesNotExist,
	DatabaseError,
}

struct ProjectRepository {}

impl IProjectsRepository for ProjectRepository {
	fn create(&self, project: Project) -> Result<Project, ProjectsRepositoryError> {
		return match get_client().query_one(
			"SELECT * FROM create_project($1, $2, $3)",
			&[&project.name, &project.description, &project.owner.id],
		) {
			Ok(row) => {
				let error_code_or_project_id: i32 = row.get(0);
				if error_code_or_project_id == -1 {
					error!("Tried to create project with non existing user");
					Err(ProjectsUpdateError::DatabaseError)
				} else if error_code_or_project_id == -2 {
					Err(ProjectsUpdateError::IllegalName)
				} else if error_code_or_project_id == -1 {
					Err(ProjectsUpdateError::ProjectWithSameNameExists)
				}
				Ok((Project {
					id: Some(error_code_or_project_id),
					name: project.name,
					description: project.description,
					owner: project.owner,
				}))
			}
			Err(error) => {
				error!("Error occured while creating project: {}", error);
				Err(ProjectsUpdateError::DatabaseError)
			}
		};
	}

	fn update(&self, project: Project) -> Result<Project, ProjectsRepositoryError> {
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
				Some(convert_to_project(rows.first()?))
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
			WHERE psfu.user_id = $1", &[user.id]
		).unwrap_or_default().iter()
			.map(|row| convert_to_project(row))
			.collect()
	}

	fn get_all_owned_by(&self, user: &User) -> Vec<Project> {
		let rows = get_client().query(
			"SELECT p.id, p.name, p.description, u.id, u.name
			FROM projects p
			JOIN users u on u.id = p.owner_id
			WHERE owner_id = null",
			&[user.id],
		).expect("Can't fetch projects owned by some user");

		return rows.iter()
			.map(|row| convert_to_project(&row))
			.collect();
	}

	fn has_access(&self, project: &Project, user: &User) -> bool {
		match get_client().query_one("SELECT * FROM has_access_to_project($1, $2)", &[&project.id, &user.id]) {
			Ok(row) => {
				row.get(0)
			}
			Err(error) => {
				error!("Error occured while checking access to project: {}", error);
				false
			}
		}
		unimplemented!()
	}

	fn grant_access(&self, project: &Project, user: &User) -> Result<(), ProjectsRepositoryError> {
		unimplemented!()
	}

	fn revoke_access(&self, project: &Project, user: &User) -> Result<(), ProjectsRepositoryError> {
		unimplemented!()
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

pub fn grant_access(project: &Project, user: &User) -> Result<(), DatabaseError> {
	let mut client = get_client();
	let mut result = Ok(());

	match client.query_one("SELECT * FROM grant_access_to_project($1, $2", &[&project.id, &user.id]) {
		Ok(row) => {
			let result_code = row.get(0);
			if result_code < 0 {
				result = Err(DatabaseError {
					error_code: result_code,
					message: get_grant_access_to_project_message_from_code(result_code),
				})
			}
		}
		Err(_) => result = Err(DatabaseError {
			error_code: -2,
			message: String::from("Unknown error"),
		})
	}
	result
}

fn get_grant_access_to_project_message_from_code(error_code: i32) -> String {
	match error_code {
		-1 => String::from("Project does not exist"),
		_ => String::from("Unknown error")
	}
}

pub fn revoke_access(project: &Project, user: &User) -> Result<(), DatabaseError> {
	return match get_client().query_one(
		"SELECT * FROM revoke_access_to_project($1, $2)", &[&project.id, &user.id],
	) {
		Ok(result) => {
			let result_code: i32 = result.get(0);
			if result_code < 0 {
				return Err(DatabaseError {
					error_code: result_code,
					message: get_revoke_access_message_from_code(result_code),
				});
			}
			Ok(())
		}
		Err(_) => Err(DatabaseError {
			error_code: -2,
			message: String::from("Database error"),
		})
	};
}

fn get_revoke_access_message_from_code(error_code: i32) -> String {
	return match error_code {
		-1 => String::from("This user is project owner"),
		_ => String::from("Unknown error")
	};
}
