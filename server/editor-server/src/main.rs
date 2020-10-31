use actix_redis::RedisSession;
use actix_web::{middleware, App, HttpServer};
use editor_server::controllers::projects::{get_all_projects, delete_project, create_project, grant_access};
use editor_server::controllers::users::{register, login, logout};
use rand::Rng;
use env_logger::Env;
use time::Duration;

const REDIS_ADDR: &str = "127.0.0.1:6379";
const SERVER_ADDR: &str = "127.0.0.1:5000";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	println!("Starting server at {}", SERVER_ADDR);
	env_logger::from_env(Env::default().default_filter_or("info")).init();
	HttpServer::new(|| {
		App::new()
			.wrap(
				RedisSession::new(REDIS_ADDR, &rand::thread_rng().gen::<[u8; 32]>())
					.cookie_http_only(false)
					.cookie_name("sessiona")
					.cookie_max_age(Duration::hours(3)))
			.wrap(middleware::Logger::default())

			.service(get_all_projects)
			.service(delete_project)
			.service(create_project)
			// .service(revoke_access)
			.service(grant_access)

			.service(register)
			.service(login)
			.service(logout)
	})
		.bind(SERVER_ADDR)?
		.run()
		.await
}
