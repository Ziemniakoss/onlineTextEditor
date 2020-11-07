pub struct User {
	pub id: i32,
	pub name: String,
}

impl PartialEq for User{
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

pub struct Project {
	pub id: Option<i32>,
	pub name: String,
	pub description: String,
	pub owner: User,
}

impl Project{
	pub fn new(name:String, description:String, owner: User) -> Project{
		Project{
			id:None,
			name,
			description,
			owner
		}
	}
}

impl PartialEq for Project{
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}