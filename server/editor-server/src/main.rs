use actix_redis::RedisSession;
use actix_web::{middleware, App, HttpServer};
use editor_server::controllers::projects::{get_all_projects, delete_project, create_project};
use editor_server::controllers::users::{register, login, logout};
use rand::Rng;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let addr = "127.0.0.1:5000";
	println!("Starting server at {}", addr);
	env_logger::from_env(Env::default().default_filter_or("info")).init();
	HttpServer::new(|| {
		App::new()
			.wrap(RedisSession::new("127.0.0.1:6379", &rand::thread_rng().gen::<[u8; 32]>()).cookie_http_only(false).cookie_name("session"))
			.wrap(middleware::Logger::default())
			.service(get_all_projects)
			.service(delete_project)
			.service(create_project)
			.service(register)
			.service(login)
			.service(logout)
	})
		.bind(addr)?
		.run()
		.await
}
