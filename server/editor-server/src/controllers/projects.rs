use actix_web::body::Body;
use actix_web::{get, post, delete, web, HttpRequest, HttpResponse, Result, App};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{ guard, middleware, Error, HttpServer, Responder};
use std::{env, io};

#[get("/projects")]
pub async fn get_all_projects() -> Result<HttpResponse<Body>> {
	let response = HttpResponse::Ok().body(Body::from("projetky"));
	return Ok(response);
}

#[post("/projects")]
pub async fn create_project(req: HttpRequest) -> Result<HttpResponse<Body>> {
	println!("{:#?}", req);
	let response = HttpResponse::Ok().body(Body::from("creating project"));
	return Ok(response);
}

#[delete("/projects/{id}")]
pub async fn delete_project(web::Path(id): web::Path<u32>) -> Result<HttpResponse<Body>> {
	if id == 10 {
		return Ok(HttpResponse::Ok().body("aa"));
	}
	return  Ok(HttpResponse::NotFound().body("Project not found"));
}
