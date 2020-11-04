use actix_web::{middleware, App, HttpServer};
use editor_server::controllers::projects::{get_all_projects, delete_project, create_project, grant_access};
use editor_server::controllers::users::{register, login, logout};
use rand::Rng;
use env_logger::Env;
use actix_cors::Cors;
use actix_http::cookie::SameSite;
use actix_session::CookieSession;

const SERVER_ADDR: &str = "0.0.0.0:5000";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	println!("Starting server at {}", SERVER_ADDR);
	env_logger::from_env(Env::default().default_filter_or("info")).init();
	HttpServer::new(|| {
		App::new()
			.wrap(
				CookieSession::signed(&[0; 32])//Very unsecure but for example app this is sufficent
					.secure(true)
					.same_site(SameSite::Lax)
					.name("session")
			)
			.wrap(middleware::Logger::default())
			.wrap(
				Cors::permissive()
					// .max_age(60 * 60 * 3)
					.supports_credentials()
			)
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
