use std::time::{Duration, Instant};

use actix::*;
use actix_web_actors::ws;
use crate::server;
use crate::models::User;
use crate::server::{ProjectInfoDto, ErrorMessage, FileCreated, FileDeleted};
use log::{error, info, warn};
use serde::Deserialize;


const INCOMING_CODE_NEW_FILE: &str = "1";
const INCOMING_CODE_DELETE_FILE: &str = "2";
const INCOMING_CODE_RENAME_FILE: &str = "3";
const INCOMING_CODE_GET_FILE_CONTENT: &str = "4";
const INCOMING_CODE_CHANGE_IN_FILE: &str = "5";

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct EditorSession {
	/// unique session id
	pub id: i32,
	/// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
	/// otherwise we drop connection.
	pub hb: Instant,
	pub project_id: i32,
	pub user: User,
	/// Editor server
	pub addr: Addr<server::EditorServer>,
}

#[derive(Message)]
#[rtype(i32)]
pub struct Connect {
	pub addr: Addr<EditorSession>,
	pub project_id: i32,
	pub user: User,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
	pub session_id: i32
}

//TODO
#[derive(Message)]
#[rtype(result = "()")]
pub struct IncomingChange {
	pub session_id: i32,
	pub file_id: i32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct FileCreationRequest {
	pub session_id: i32,
	pub filename: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct FileDeletionRequest {
	pub session_id: i32,
	pub file_id: i32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct FileNameChangeRequest {
	pub session_id: i32,
	pub file_id: i32,
	pub new_filename: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct FileContentRequest {
	pub session_id: i32,
	pub file_id: i32
}


impl Actor for EditorSession {
	type Context = ws::WebsocketContext<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		// we'll start heartbeat process on session start.
		self.heart_beat(ctx);

		let addr = ctx.address();
		self.addr
			.send(Connect {
				addr,
				project_id: self.project_id,
				user: self.user.clone(),
			})
			.into_actor(self)
			.then(|res, act, ctx| {
				match res {
					Ok(res) => act.id = res,
					// something is wrong with chat server
					_ => ctx.stop(),
				}
				fut::ready(())
			})
			.wait(ctx);
	}

	fn stopping(&mut self, _: &mut Self::Context) -> Running {
		// notify chat server
		self.addr.do_send(Disconnect { session_id: self.id });
		Running::Stop
	}
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<server::Message> for EditorSession {
	type Result = ();

	fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
		ctx.text(msg.0);
	}
}

impl Handler<server::ProjectInfoDto> for EditorSession {
	type Result = ();

	fn handle(&mut self, msg: ProjectInfoDto, ctx: &mut Self::Context) -> Self::Result {
		match serde_json::to_string(&msg) {
			Ok(json) => ctx.text(format!("9{}", json)),
			Err(err) => {
				error!("Error while serializing ProjectInfoDto: {}", err);
			}
		}
	}
}

impl Handler<server::FileCreated> for EditorSession {
	type Result = ();

	fn handle(&mut self, msg: FileCreated, ctx: &mut Self::Context) -> Self::Result {
		ctx.text(format!("3{} {}", msg.id, msg.name));
	}
}

impl Handler<server::ErrorMessage> for EditorSession {
	type Result = ();

	fn handle(&mut self, msg: ErrorMessage, ctx: &mut Self::Context) -> Self::Result {
		ctx.text(format!("a{}", msg.msg));
	}
}

impl Handler<server::FileDeleted> for EditorSession {
	type Result = ();

	fn handle(&mut self, msg: FileDeleted, ctx: &mut Self::Context) -> Self::Result {
		ctx.text(format!("4{}", msg.id));
	}
}

impl Handler<server::FileContent> for EditorSession{
	type Result =();

	fn handle(&mut self, msg: server::FileContent, ctx: &mut Self::Context) -> Self::Result {
		ctx.text(format!("5{} {}", msg.file_id, msg.content));
	}
}

#[derive(Debug, Deserialize)]
pub struct FileChange {
	pub start: Position,
	pub end: Position,
	pub file_id: i32,
	pub lines: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Position {
	pub row: u32,
	pub column: u32,
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for EditorSession {
	fn handle(
		&mut self,
		msg: Result<ws::Message, ws::ProtocolError>,
		ctx: &mut Self::Context,
	) {
		let msg = match msg {
			Err(error) => {
				println!("Error un stream handler: {:?}", error);
				ctx.stop();
				return;
			}
			Ok(msg) => msg,
		};

		// println!("WEBSOCKET MESSAGE: {:?}", msg);
		match msg {
			ws::Message::Ping(msg) => {
				self.hb = Instant::now();
				ctx.pong(&msg);
			}
			ws::Message::Pong(_) => {
				self.hb = Instant::now();
			}
			ws::Message::Text(text) => {
				self.parse_message_and_send_to_server(text, ctx);
			}
			ws::Message::Binary(_) => println!("Unexpected binary"),
			ws::Message::Close(reason) => {
				ctx.close(reason);
				ctx.stop();
			}
			ws::Message::Continuation(_) => {
				ctx.stop();
			}
			ws::Message::Nop => (),
		}
	}
}

impl EditorSession {
	fn parse_message_and_send_to_server(&self, message: String, _: &mut ws::WebsocketContext<Self>) {
		if message.len() == 0 {
			println!("Empty message with no code");
		}
		let (incoming_code, incoming_message) = message.split_at(1);
		match incoming_code {
			INCOMING_CODE_NEW_FILE => {
				println!("New file req, file name {}", incoming_message);
				self.addr.do_send(FileCreationRequest {
					session_id: self.id,
					filename: incoming_message.to_owned(),
				})
			}
			INCOMING_CODE_DELETE_FILE => {
				let file_id;
				match incoming_message.parse::<i32>() {
					Ok(id) => file_id = id,
					Err(_) => {
						println!("Unparsable file id");
						return;
					}
				}
				info!("Session {} editing project {} requested deletion of file {}", self.id, self.project_id, file_id);
				self.addr.do_send(FileDeletionRequest {
					session_id: self.id,
					file_id,
				});
			}
			INCOMING_CODE_RENAME_FILE => {
				println!("Rename file request");
			}
			INCOMING_CODE_CHANGE_IN_FILE => {
				info!("Incoming change in file")
			}
			INCOMING_CODE_GET_FILE_CONTENT =>{
				info!("Incoming file content request");
				let file_id;
				match incoming_message.parse::<i32>(){
					Ok(id) => file_id = id,
					Err(_) => {
						warn!("Session {} sent file content request with unparsable file id {}", self.id, incoming_message);
						return;
					}
				}
				self.addr.do_send(FileContentRequest{
					session_id: self.id,
					file_id
				})
			}
			_ => {
				println!("Unknown first char: {} of message in session {}", incoming_code, self.id);
			}
		}
	}

	/// helper method that sends ping to client every second.
	///
	/// also this method checks heartbeats from client
	fn heart_beat(&self, ctx: &mut ws::WebsocketContext<Self>) {
		ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
			// check client heartbeats
			if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
				// heartbeat timed out
				println!("Websocket Client heartbeat failed, disconnecting!");
				// notify chat server
				act.addr.do_send(Disconnect { session_id: act.id });
				// stop actor
				ctx.stop();
				// don't try to send a ping
				return;
			}
			ctx.ping(b"");
		});
	}
}
