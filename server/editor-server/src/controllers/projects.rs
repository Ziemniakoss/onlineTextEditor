use actix_web::body::{Body};
use actix_web::{get, post, delete, web, HttpResponse, Result};
use actix_session::Session;
use crate::session_manager::get_user_id;
use actix_http::http::StatusCode;
use serde::Deserialize;
use crate::repositories::projects::{Project, get_project};
use crate::repositories::users::{User, get_user};
use log::{error, info, warn};

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
		None => {}//TODO
	}
	let response = HttpResponse::Ok().body(Body::from("projetky"));
	return Ok(response);
}

#[post("/projects")]
pub async fn create_project(project_dto: web::Json<ProjectCreationDto>, session: Session) -> Result<HttpResponse<Body>> {
	let user_id: i32;
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
		id: -100,
		name: project_dto.name.clone(),
		description: project_dto.description.clone(),
		owner: User { id: user_id, name: String::new() },
	}) {
		Ok(project_id) => {
			Ok(response_builder.body(format!("{{\"id\": \"{}\"}}", project_id)))
		}
		Err(error) => Ok(response_builder.status(StatusCode::BAD_REQUEST).body(format!("{{\"message\": {}}}", error.message)))
	};
}

#[delete("/projects/{id}")]
pub async fn delete_project(web::Path(id): web::Path<u32>) -> Result<HttpResponse<Body>> {
	if id == 10 {
		return Ok(HttpResponse::Ok().body("aa"));
	}
	return Ok(HttpResponse::NotFound().body("Project not found"));
}
// TODO
// #[delete("/projects/{id}/access/{user_id}")]
// pub async fn revoke_access(web::Path(id): web::Path<i32>, session: Session) -> Result<HttpResponse<Body>> {
// }

#[post("/projects/{id}/access/{user_id}")]
pub async fn grant_access(web::Path((id, user_id)): web::Path<(i32, i32)>, session: Session) -> Result<HttpResponse<Body>> {
	let mut response_builder = HttpResponse::build(StatusCode::OK);
	let current_user_id: i32;
	match get_user_id(&session) {
		Some(user_id_from_session) => {
			current_user_id = user_id_from_session;
		}
		None => {
			warn!("Unauthorized user wanted to grant access to project {}", id);
			return Ok(response_builder
				.status(StatusCode::UNAUTHORIZED)
				.body("Please log in"));
		}
	}
	let user = get_user(user_id);
	if let None = user {
		warn!("User with id {} wanted to grant non-existing user with id {} access to project {}", current_user_id, user_id, id);
		return Ok(response_builder
			.status(StatusCode::NOT_FOUND)
			.body(Body::from("User does not exist")));
	}
	let project = get_project(id);
	if let None = project {
		warn!("User with id {} wanted to grant user with id {} access to non-existing project with id {}", current_user_id, user_id, id);
		return Ok(response_builder
			.status(StatusCode::NOT_FOUND)
			.body(Body::from("Project does not exist")));
	}
	let project = project.unwrap();
	if project.owner.id == current_user_id {
		warn!("User with id {} wanted to grant access to project with id {} to itself", current_user_id, id);
		return Ok(response_builder
			.status(StatusCode::BAD_REQUEST)
			.body("You are owner of this project"));
	}
	return match crate::repositories::projects::grant_access(&project, &user.unwrap()) {
		Ok(_) => {
			info!("User with id {} granted user with id {} access to {}", current_user_id, user_id, id);
			Ok(response_builder.body("granted access to user"))
		}
		Err(error) => {
			error!("Error occurred while granting access to project: {}", error.message);
			Ok(response_builder
				.status(StatusCode::INTERNAL_SERVER_ERROR)
				.body("Unknown error occurred"))
		}
	};
}
