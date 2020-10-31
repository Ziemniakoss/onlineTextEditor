use actix_session::Session;
use actix_http::Error;
use log::error;

pub fn get_user_id(session: &Session) -> Option<i32> {
	return match session.get("user_id") {
		Ok(possible_user_id) => possible_user_id,
		Err(error) => {
			println!("Failed to access user id from session: {}", error);
			None
		}
	};
}

pub fn set_user_id(session: &Session, user_id: i32) {
	match session.set("user_id", user_id) {
		Err(error) => {
			error!("Error occured while trying to set user id in session: {}", error);
		}
		_ => {}
	}
}