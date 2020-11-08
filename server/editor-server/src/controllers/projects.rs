use actix_web::body::{Body};
use actix_web::{get, post, delete, web, HttpResponse, Result};
use actix_session::Session;
use crate::session_manager::get_user_id;
use actix_http::http::StatusCode;
use serde::Deserialize;
use crate::repositories::users::{get_user};
use crate::services::projects;
use log::{error, warn};
use crate::services::projects::{SaveError, AccessGrantingError, GetError, AccessRevokingError};
use crate::models::{Project, User};

#[derive(Deserialize, Debug)]
pub struct ProjectCreationDto {
	name: String,
	description: String,
}

#[get("/projects/my")]
pub async fn get_my_projects(session: Session) -> HttpResponse<Body> {
	let mut response_builder = HttpResponse::build(StatusCode::OK);
	let user_id: i32;
	match get_user_id(&session) {
		Some(user_id_in_session) => {
			user_id = user_id_in_session;
		}
		None => {
			error!("Not logged in user tried to fetch his/its/its projects");
			return response_builder.status(StatusCode::UNAUTHORIZED)
				.body(Body::from("Please log in"));
		}
	}
	let user = get_user(user_id).unwrap();
	let service = projects::new(user);
	return response_builder.json(service.get_owned_projects());
}

#[get("/projects/shared-for-me")]
pub async fn get_projects_shared_for_me(session: Session) -> HttpResponse<Body> {
	let mut response_builder = HttpResponse::build(StatusCode::OK);
	let user_id;
	match get_user_id(&session) {
		Some(user_id_in_session) => user_id = user_id_in_session,
		None => {
			error!("Not logged in user tried to fetch his/its/its projects");
			return response_builder.status(StatusCode::UNAUTHORIZED)
				.body(Body::from("Please log in"));
		}
	}
	let user;
	match get_user(user_id) {
		None => {
			warn!("Recived request from session with not existing user, possible attack");//TODO ip bla bla bla
			return response_builder
				.status(StatusCode::INTERNAL_SERVER_ERROR)
				.body("You don't exist");
		}
		Some(u) => user = u
	}
	let service = projects::new(user);
	return response_builder.json(service.get_projects_shared_to_user());
}


#[get("/projects/{id}")]
pub async fn get_project(session: Session, web::Path(id): web::Path<i32>) -> HttpResponse<Body> {
	let mut response_builder = HttpResponse::build(StatusCode::OK);
	let user;
	match get_user_id(&session) {
		Some(user_id_in_session) => {
			match get_user(user_id_in_session) {
				None => {
					warn!("Recived request from session with not existing user, possible attack");//TODO ip bla bla bla
					return response_builder
						.status(StatusCode::INTERNAL_SERVER_ERROR)
						.body("You don't exist");
				}
				Some(u) => user = u
			}
		}
		None => {
			error!("Not logged in user tried to fetch his/its/its projects");
			return response_builder
				.status(StatusCode::UNAUTHORIZED)
				.body(Body::from("Please log in"));
		}
	}
	let service = projects::new(user);
	return match service.get(id) {
		Ok(project) => response_builder.json(project),
		Err(_) => response_builder
			.status(StatusCode::NOT_FOUND)
			.body("Project does not eist or you dont have access to it")
	};
}

#[post("/projects")]
pub async fn create_project(project_dto: web::Json<ProjectCreationDto>, session: Session) -> HttpResponse<Body> {
	let mut response_builder = HttpResponse::build(StatusCode::OK);
	let user;
	match get_user_id(&session) {
		Some(user_id) => {
			match get_user(user_id) {
				Some(u) => user = u,
				None => {
					warn!("Session with not existing user");
					return response_builder
						.status(StatusCode::INTERNAL_SERVER_ERROR)
						.body("You dont exist");
				}
			}
		}
		None => {
			return response_builder
				.status(StatusCode::UNAUTHORIZED)
				.body(Body::from("{\"message\":\"You have to be logged in to create projects\"}"));
		}
	}
	let service = projects::new(user);
	return match service.create(project_dto.name.clone(), project_dto.description.clone()) {
		Ok(project) => {
			response_builder.json(project)
		}
		Err(error) => {
			response_builder.status(StatusCode::BAD_REQUEST);
			match error {
				SaveError::InvalidName => response_builder.body("Invalid name"),
				SaveError::ProjectWithSameNaeAlreadyExists => response_builder.body("You have project with same name"),
				_ => {
					error!("Unknon error occured while creating project");
					response_builder.status(StatusCode::INTERNAL_SERVER_ERROR).finish()
				}
			}
		}
	};
}

#[delete("/projects/{id}")]
pub async fn delete_project(web::Path(id): web::Path<u32>, session: Session) -> Result<HttpResponse<Body>> {
	todo!()
}

#[delete("/projects/{id}/access/{user_id}")]
pub async fn revoke_access(web::Path((id, user_id)): web::Path<(i32, i32)>, session: Session) -> HttpResponse<Body> {
	let user;
	let mut response_builder = HttpResponse::build(StatusCode::OK);
	match get_user_id(&session) {
		Some(user_id) => {
			user = get_user(user_id).unwrap();
		}
		None => {
			return response_builder
				.status(StatusCode::UNAUTHORIZED)
				.body("Please log in");
		}
	}
	let user_to_grant_access;
	match get_user(user_id) {
		Some(u) => user_to_grant_access = u,
		None => return response_builder
			.status(StatusCode::NOT_FOUND)
			.body("User does not exist")
	}
	let project;
	let service = projects::new(user);
	match service.get(id) {
		Ok(p) => project = p,
		Err(_) => {
			return response_builder
				.status(StatusCode::NOT_FOUND)
				.body("Project does not exist or you dont have access to id");
		}
	}
	return match service.revoke_access(&project, &user_to_grant_access){
		Ok(_) => response_builder.body("ok"),
		Err(error) =>{
			match error {
				AccessRevokingError::IsOwner => response_builder
					.status(StatusCode::BAD_REQUEST)
					.body("This user is owner of project"),
				AccessRevokingError::UserIsNotOwner => response_builder
					.status(StatusCode::FORBIDDEN)
					.body("You are not owner of this project"),
				AccessRevokingError::UserDoesNotExists => response_builder
					.status(StatusCode::NOT_FOUND)
					.body("This user does not exist")
			}
		}
	}
}

#[post("/projects/{id}/access/{user_id}")]
pub async fn grant_access(web::Path((id, user_id)): web::Path<(i32, i32)>, session: Session) -> HttpResponse<Body> {
	let mut response_builder = HttpResponse::build(StatusCode::OK);
	let user;
	match get_user_id(&session) {
		Some(user_id_from_session) => {
			user = get_user(user_id_from_session).expect("User does not exist");
		}
		None => {
			warn!("Unauthorized user wanted to grant access to project {}", id);//TODO ip bla bla bla
			return response_builder
				.status(StatusCode::UNAUTHORIZED)
				.body("Please log in");
		}
	}
	let service = projects::new(user);
	let project;
	match service.get(id) {
		Ok(p) => project = p,
		Err(_) => {
			return response_builder
				.status(StatusCode::NOT_FOUND)
				.body("Project does not exist or you dont have access to it");
		}
	}
	let user_to_grant;
	match get_user(user_id) {
		Some(u) => user_to_grant = u,
		None => return response_builder
			.status(StatusCode::NOT_FOUND)
			.body("User does not exists")
	}
	return match service.grant_access(&project, &user_to_grant) {
		Ok(_) => response_builder.body("Ok"),
		Err(error) => {
			match error {
				AccessGrantingError::NotOwner => response_builder
					.status(StatusCode::NOT_FOUND)
					.body("Project does not exist or you dont have access to it")
			}
		}
	};
}
