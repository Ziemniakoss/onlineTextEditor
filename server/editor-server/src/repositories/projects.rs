use crate::repositories::users::User;
use crate::repositories::{get_client, DatabaseError};
use postgres::Row;
use log::{error, info};

pub struct Project {
	pub id: i32,
	pub name: String,
	pub description: String,
	pub owner: User,
}

pub fn create_project(project: Project) -> Result<i32, DatabaseError> {
	return match get_client().query_one(
		"SELECT * FROM create_project($1, $2, $3)",
		&[&project.name, &project.description, &project.owner.id],
	) {
		Ok(row) => {
			let error_code_or_project_id: i32 = row.get(0);
			return if error_code_or_project_id < 0 {
				Err(DatabaseError {
					error_code: error_code_or_project_id,
					message: get_project_creation_message_from_code(error_code_or_project_id),
				})
			} else {
				Ok(row.get(0))
			}
		}
		Err(error) => {
			error!("Error occured {}", error);
			Err(DatabaseError {
				message: String::from("Unknown error"),
				error_code: -4,
			})
		}
	}
}

pub fn get_project(id: i32) -> Option<Project> {
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

fn convert_to_project(row: &Row) -> Project {
	let owner = User {
		id: row.get(3),
		name: row.get(4),
	};
	Project {
		id: row.get(0),
		name: row.get(1),
		description: row.get(2),
		owner,
	}
}

pub fn get_all_users_projects(user: &User) -> Vec<Project> {
	let mut client = get_client();
	match client.query(
		"SELECT p.id, p.name, p.description, u.id, u.name\
		FROM projects p\
		JOIN users u on p.owner_id = u.id \
		WHERE u.id = $1 ORDER BY p.name", &[&user.id],
	) {
		Ok(rows) => {
			let projects: Vec<Project> = rows.iter().map(|row| convert_to_project(row)).collect();
			info!("User {} fetched his {} projects", user.id, projects.len());
			projects
		}
		Err(error) => {
			error!("Error occured while fetching all user's projects: {}", error);
			vec![]
		}
	}
}

pub fn get_all_projects_shared_to_user(user: &User) -> Vec<Project> { vec![] }

pub fn has_access(project: &Project, user: &User) -> Result<bool, DatabaseError> {
	let mut client = get_client();
	match client.query_one("SELECT * FROM has_access_to_project($1, $2)", &[&project.id, &user.id]) {
		Ok(row) => {
			Ok(row.get(0))
		}
		Err(error) => {
			error!("Error occured while checking access to project: {}", error);
			Err(DatabaseError {
				error_code: -1,
				message: String::from("Unknown error"),
			})
		}
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

fn get_project_creation_message_from_code(error_code: i32) -> String {
	return match error_code {
		-1 => String::from("User marked as owner does not exist"),
		-2 => String::from("Project name can't be empty"),
		-3 => String::from("This user already has project with this name"),
		_ => format!("Unknown error code: {}", error_code)
	};
}