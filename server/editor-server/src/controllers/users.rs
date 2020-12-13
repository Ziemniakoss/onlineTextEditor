use actix_web::body::Body;
use actix_web::{post, web, HttpResponse, Result};
use crate::repositories::users::{create_user, login as try_login};
use actix_web::http::StatusCode;
use serde::Deserialize;
use actix_session::Session;
use crate::session_manager::{get_user_id, set_user_id};
use log::{ info};


#[post("/login")]
pub async fn login(user_dto: web::Json<UserAuthorizationDto>, session: Session) -> Result<HttpResponse<Body>> {
	let mut response_builder = HttpResponse::build(StatusCode::OK);
	return match try_login(&user_dto.username, &user_dto.password) {
		Ok(user) => {
			info!("User logged in: {}", user.name);
			set_user_id(&session, user.id);
			session.renew();
			Ok(response_builder.body("hello"))
		}
		Err(_) => {
			info!("User tried to log in with incorrect password or login");//TODO ip bla bla bla
			Ok(response_builder.status(StatusCode::UNAUTHORIZED).body("Invalid login"))
		}
	};
}

#[post("/logout")]
pub async fn logout(session: Session) -> Result<HttpResponse<Body>> {
	let mut response_builder = HttpResponse::build(StatusCode::OK);
	return match get_user_id(&session) {
		Some(user_id) => {
			session.purge();
			info!("User with id {} logged out", user_id);
			Ok(response_builder.body("Logged out"))
		}
		None => Ok(response_builder.status(StatusCode::UNAUTHORIZED).body("Nu allowed"))
	};
}

#[post("/register")]
pub async fn register(user_dto: web::Json<UserAuthorizationDto>) -> Result<HttpResponse<Body>> {
	println!("{:#?}", user_dto);
	let mut response_builder = HttpResponse::build(StatusCode::OK);
	response_builder.header("Access-Control-Allow-Origin","*");
	return match create_user(&user_dto.username, &user_dto.password) {
		Ok(user) => {
			match serde_json::to_string(&user) {
				Ok(json) => Ok(response_builder.body(json)),
				Err(stringifing_error) => {
					println!("Error occured while serializing user: {}", stringifing_error);
					Ok(response_builder.status(StatusCode::INTERNAL_SERVER_ERROR).body("Server error occured"))
				}
			}
		}
		Err(error_message) => {
			Ok(response_builder.status(StatusCode::BAD_REQUEST).body(error_message))
		}
	};
}

#[derive(Debug, Deserialize)]
pub struct UserAuthorizationDto {
	pub username: String,
	pub password: String,
}