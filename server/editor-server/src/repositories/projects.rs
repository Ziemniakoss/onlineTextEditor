use crate::repositories::users::User;

pub struct Project{
	name:String,
	description:Option<String>,
	owner:User
}