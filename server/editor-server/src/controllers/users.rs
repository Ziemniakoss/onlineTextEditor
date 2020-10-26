use actix_web::body::Body;
use actix_web::{post, web, HttpRequest, HttpResponse, Result, HttpMessage};
use crate::repositories::users::create_user;
use actix_web::http::StatusCode;
use futures::future::err;
use serde::Deserialize;

// TODO
// #[post("/login")]
// pub async fn login(req: HttpRequest) -> Result<HttpResponse<Body>>{
//
// }
//
#[post("/register")]
pub async fn register(user_dto : web::Json<UserDto>) -> Result<HttpResponse<Body>>{

	println!("{:#?}", user_dto);
	let mut response_builder = HttpResponse::build(StatusCode::OK);
	return match create_user(&user_dto.username, &user_dto.password) {
		Ok(user) => {
			match serde_json::to_string(&user) {
				Ok(json) =>Ok(response_builder.body(json)),
				Err(stringifing_error) =>{
					println!("Error occured while serializing user: {}", stringifing_error);
					Ok(response_builder.status(StatusCode::INTERNAL_SERVER_ERROR).body("Server error occured"))
				}
			}
		}
		Err(error_message) => {
			Ok(response_builder.status(StatusCode::BAD_REQUEST).body(error_message))
		}
	}
}
#[derive(Debug, Deserialize)]
struct UserDto{
	username:String,
	password:String
}