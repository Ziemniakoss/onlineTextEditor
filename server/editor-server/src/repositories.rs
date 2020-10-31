use postgres::{Client, NoTls};
use log::error;
pub mod users;
pub mod projects;
pub mod files;

const USERNAME: &str = "postgres";
const PASSWORD: &str = "postgres";
const HOST: &str = "127.0.0.1";
const DB_NAME: &str = "studres";

fn get_client() -> Client {
	let client = Client::configure()
		.dbname(DB_NAME)
		.password(PASSWORD)
		.user(USERNAME)
		.host(HOST)
		.connect(NoTls);
	match client {
		Ok(c) => return c,
		Err(error) => {
			error!("Ala ma kota");
			println!("AAAAAAAAAAAAAAA {}", error);
			panic!(error)
		}
	}
}