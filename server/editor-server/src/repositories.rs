use postgres::{Client, Error, NoTls};

pub mod users;
pub mod projects;
pub mod files;

fn get_client() -> Client{

	let client = Client::configure()
		.dbname("studres")
		.password("postgres")
		.user("postgres")
		.host("localhost")
		.connect(NoTls);
	match client{
		Ok(c) =>return c,
		Err(error) => panic!(error)
	}
}