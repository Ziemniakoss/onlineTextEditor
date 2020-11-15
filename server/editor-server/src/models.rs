use serde::Serialize;
use postgres::types::IsNull::No;

#[derive(Serialize)]
pub struct User {
	pub id: i32,
	pub name: String,
}

impl PartialEq for User {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

#[derive(Serialize)]
pub struct Project {
	pub id: Option<i32>,
	pub name: String,
	pub description: String,
	pub owner: User,
}

impl Project {
	pub fn new(name: String, description: String, owner: User) -> Project {
		Project {
			id: None,
			name: name.to_owned(),
			description: description.to_owned(),
			owner: User { id: owner.id, name: owner.name.clone() },
		}
	}
}

impl PartialEq for Project {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

#[derive(Serialize)]
pub struct ProjectFile {
	pub id: Option<i32>,
	pub name: String,
	pub project_id: i32,
}

impl PartialEq for ProjectFile {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl ProjectFile {
	pub fn new(name: String, project: &Project) -> ProjectFile {
		ProjectFile {
			id: None,
			name,
			project_id: project.id.expect("Can't create file for non existing project"),
		}
	}
}

