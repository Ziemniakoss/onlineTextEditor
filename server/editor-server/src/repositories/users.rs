use crate::repositories::get_client;
use postgres::Row;
use log::{error};
use crate::models::User;

pub fn create_user(login: &String, password: &String) -> Result<User, String> {
	let mut client = get_client();
	if user_exists(login) {
		return Err(String::from("User with this username already exists"));
	}
	match client.query_one("INSERT INTO users (name, password_hash) VALUES ($1, md5($2)) RETURNING id", &[login, password]) {
		Ok(row) => {
			println!("Created user {}", login);
			Ok(User { id: row.get(0), name: login.clone() })
		}
		Err(error) => {
			println!("Error occured while createing user: {}", error);
			Err(String::from("Unknown database error"))
		}
	}
}

fn user_exists(username: &String) -> bool {
	get_client().query_one("SELECT(EXISTS(SELECT id FROM users WHERE name = $1))", &[username])
		.expect("error occured while fetching data").get(0)
}

pub fn login(username: &String, password: &String) -> Result<User, String> {
	match get_client()
		.query_one("SELECT id, name FROM users WHERE name = $1 AND password_hash = md5($2)",
				   &[username, password]) {
		Ok(row) => Ok(User { id: row.get(0), name: row.get(1) })
		,
		Err(error) => {
			println!("Error occured while loggging: {}", error);
			Err(String::from("Unknown server error"))
		}
	}
}

pub fn get_user(id: i32) -> Option<User> {
	return match get_client().query_one("SELECT id, name FROM users WHERE id = $1", &[&id]) {
		Ok(row) => {
			Some(convert_to_user(&row))
		}
		Err(error) => {
			error!("Error occured while fetching user by id {}: {}", id, error);
			None
		}
	};
}

fn convert_to_user(row: &Row) -> User {
	User { id: row.get(0), name: row.get(1) }
}
