use actix_web::body::Body;
use actix_web::{post, web, HttpResponse, Result};
use crate::repositories::users::{create_user, login as try_login};
use actix_web::http::StatusCode;
use serde::Deserialize;
use actix_session::Session;

#[post("/login")]
pub async fn login(user_dto: web::Json<UserDto>, session: Session) -> Result<HttpResponse<Body>> {
	let mut response_builder = HttpResponse::build(StatusCode::OK);
	return match try_login(&user_dto.username, &user_dto.password) {
		Ok(user) => {
			println!("User {} logged in", user.name);
			session.set("user_id", user.id);
			session.renew();
			Ok(response_builder.body("hello"))
		}
		Err(e) => {
			println!("{}", e);
			Ok(response_builder.status(StatusCode::UNAUTHORIZED).body("Invalid login"))
		}
	};
}

#[post("/logout")]
pub async fn logout(session: Session) -> Result<HttpResponse<Body>> {
	let mut response_builder = HttpResponse::build(StatusCode::OK);
	let id: Option<i32> = session.get("user_id")?;
	return if let Some(_) = id {
		session.purge();
		Ok(response_builder.body("Logged out"))
	} else {
		Ok(response_builder.status(StatusCode::UNAUTHORIZED).body("Nu allowed"))
	};
}

#[post("/register")]
pub async fn register(user_dto: web::Json<UserDto>) -> Result<HttpResponse<Body>> {
	println!("{:#?}", user_dto);
	let mut response_builder = HttpResponse::build(StatusCode::OK);
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
struct UserDto {
	username: String,
	password: String,
}