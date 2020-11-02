use postgres::{Client, NoTls};
use log::error;
use std::env;

pub mod users;
pub mod projects;
pub mod files;

fn get_client() -> Client {
	let password = env::var("ONLINE_EDITOR_DATABASE_PASSWORD").expect("SET \"ONLINE_EDITOR_DATABASE_PASSWORD\" env variable");
	let username = env::var("ONLINE_EDITOR_DATABASE_USERNAME").expect("SET \"ONLINE_EDITOR_DATABASE_USERNAME\" env variable");
	let host = env::var("ONLINE_EDITOR_DATABASE_HOST").expect("SET \"ONLINE_EDITOR_DATABASE_HOST\" env variable");
	let dbname = env::var("ONLINE_EDITOR_DATABASE_DBNAME").expect("SET \"ONLINE_EDITOR_DATABASE_DBNAME\" env variable");

	let client = Client::configure()
		.dbname(&dbname)
		.password(password)
		.user(&username)
		.host(&host)
		.connect(NoTls);
	match client {
		Ok(c) => return c,
		Err(error) => {
			error!("Error occured while trying to connect to postgres database: {}", error);
			panic!(error)
		}
	}
}

pub struct DatabaseError {
	pub error_code: i32,
	pub message: String,
}