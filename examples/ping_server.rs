//! Server side of script
use easy_sockets::{
    error::{deserialize_error, serialize_error, ErrorCode},
    start_server, tokio, Deserialize, Serialize, ServerConn,
};

/// Error codes for server client connection
enum PingError {
    BadRequest,
    InternalServerError,
}
impl ErrorCode for PingError {
    fn to_code(&self) -> u16 {
        match self {
            PingError::BadRequest => 400,
            PingError::InternalServerError => 500,
        }
    }

    fn from_code(code: u16) -> Option<Self> {
        Some(match code {
            400 => PingError::BadRequest,
            500 => PingError::InternalServerError,
            _ => return None,
        })
    }

    fn message(&self) -> &'static str {
        match self {
            PingError::BadRequest => "Request provided was invalid.",
            PingError::InternalServerError => "Server encountered unexpected internal error.",
        }
    }
}

/// Message that a Client sends
#[derive(Serialize, Deserialize)]
enum ClientMsg {
    Ping(String),
}

/// Message that the server sends
#[derive(Serialize, Deserialize)]
enum ServerMsg {
    #[serde(
        serialize_with = "serialize_error",
        deserialize_with = "deserialize_error"
    )]
    Error(PingError),
    Ping(String),
}

/// An instance between a server and client
struct ServerInstance {
    response: String,
}
impl ServerConn for ServerInstance {
    type ClientMsg = ClientMsg;
    type ServerMsg = ServerMsg;

    fn handle_message(&mut self, message: Self::ClientMsg) -> Self::ServerMsg {
        match message {
            ClientMsg::Ping(message) => {
                println!("Received From Client: {}", message);
                ServerMsg::Ping(self.response.clone())
            }
        }
    }

    fn new() -> Self {
        Self {
            response: "Hello Client".into(),
        }
    }
}

#[tokio::main]
async fn main() {
    let _ = start_server::<ServerInstance>("127.0.0.1:8000").await;
}
