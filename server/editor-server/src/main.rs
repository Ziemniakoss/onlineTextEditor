use actix_session::{CookieSession, Session};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result, Responder};
use std::{env, io};
use editor_server::controllers::projects::{get_all_projects, delete_project, create_project};
use postgres::{Client, NoTls};
use editor_server::controllers::users::register;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let addr = "127.0.0.1:5000";
	println!("Starting server at {}", addr);
	HttpServer::new(|| {
		App::new()
			.service(get_all_projects)
			.service(delete_project)
			.service(create_project)
			.service(register)
	})
		.bind(addr)?
		.run()
		.await
}
