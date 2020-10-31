use crate::repositories::users::User;
use crate::repositories::get_client;
use postgres::{Error, Row};
use log::{error, info};
use futures::future::err;

pub struct Project {
	pub name: String,
	pub description: String,
	pub owner: User,
}

pub fn create_project(project: Project) -> Result<i32, String> {
	let mut client = get_client();
	let mut result: Result<i32, String>;
	match client.query_one("SELECT * FROM create_project($1, $2, $3)",
						   &[&project.name, &project.description, &project.owner.id]) {		Ok(row) => {
			let error_code_or_project_id :i32= row.get(0);
			if error_code_or_project_id < 0 {
				result = Err(get_project_creation_message_from_code(error_code_or_project_id));
			}else {
				result = Ok(row.get(0));
			}
		}
		Err(error) => {
			error!("Error occured {}", error);
			result = Err(error.to_string());
		}
	}
	client.close();
	result
}

fn get_project_creation_message_from_code(error_code: i32) -> String {
	return match error_code {
		-1 => String::from("User marked as owner does not exist"),
		-2 => String::from("Project name can't be empty"),
		-3 => String::from("This user already has project with this name"),
		_ => format!("Unknown error code: {}", error_code)
	}
}