use actix_web::{middleware, App, HttpServer};
use editor_server::controllers::projects;
use editor_server::controllers::users;
use env_logger::Env;
use actix_cors::Cors;
use actix_http::cookie::SameSite;
use actix_session::CookieSession;
use log::{info};
use editor_server::server::EditorServer;
use actix::Actor;


const SERVER_ADDR: &str = "0.0.0.0:5000";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	env_logger::from_env(Env::default()
		.default_filter_or("info"))
		.init();
	info!("Starting server at {}", SERVER_ADDR);

	let server = EditorServer::default().start();
	HttpServer::new(move || {
		App::new()
			.wrap(
				CookieSession::signed(&[0; 32])//Very unsecure but for example app this is sufficent
					.secure(true)
					.http_only(true)
					.same_site(SameSite::Lax)
					.name("session")
			)
			.wrap(middleware::Logger::default())
			.wrap(
				Cors::permissive()
					.supports_credentials()
			)
			.service(projects::get_my_projects)
			.service(projects::get_projects_shared_for_me)
			.service(projects::delete_project)
			.service(projects::create_project)
			.service(projects::grant_access)
			.service(projects::revoke_access)
			.service(projects::begin_editor_session)
			.data(server.clone())

			.service(users::register)
			.service(users::login)
			.service(users::logout)
	})
		.bind(SERVER_ADDR)?
		.run()
		.await
}
