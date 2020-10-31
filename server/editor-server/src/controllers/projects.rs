use actix_web::body::Body;
use actix_web::{get, post, delete, web, HttpResponse, Result};
use actix_session::Session;
use crate::session_manager::get_user_id;
use actix_http::http::StatusCode;
use serde::Deserialize;
use crate::repositories::projects::Project;
use crate::repositories::users::User;

#[derive(Deserialize, Debug)]
pub struct ProjectCreationDto {
	name: String,
	description: String,
}

#[get("/projects")]
pub async fn get_all_projects(session: Session) -> Result<HttpResponse<Body>> {
	let response_builder = HttpResponse::build(StatusCode::OK);
	match get_user_id(&session) {
		Some(user_id) => {}
		None => {}
	}
	let response = HttpResponse::Ok().body(Body::from("projetky"));
	return Ok(response);
}

#[post("/projects")]
pub async fn create_project(project_dto: web::Json<ProjectCreationDto>, session: Session) -> Result<HttpResponse<Body>> {
	let mut user_id: i32;
	let mut response_builder = HttpResponse::build(StatusCode::OK);
	match get_user_id(&session) {
		Some(user_id_from_session) => {
			user_id = user_id_from_session;
		}
		None => {
			return Ok(response_builder
				.status(StatusCode::UNAUTHORIZED)
				.body(Body::from("{\"message\":\"You have to be logged in to create projects\"}")));
		}
	}
	return match crate::repositories::projects::create_project(Project {
		name: project_dto.name.clone(),
		description: project_dto.description.clone(),
		owner: User { id: user_id, name: String::new() },
	}) {
		Ok(project_id) => {
			Ok(response_builder.body(format!("{{\"id\": \"{}\"}}", project_id)))
		}
		Err(message) => Ok(response_builder.status(StatusCode::BAD_REQUEST).body(format!("{{\"message\": {}}}", message)))
	}
}

#[delete("/projects/{id}")]
pub async fn delete_project(web::Path(id): web::Path<u32>) -> Result<HttpResponse<Body>> {
	if id == 10 {
		return Ok(HttpResponse::Ok().body("aa"));
	}
	return Ok(HttpResponse::NotFound().body("Project not found"));
}
