//! Client side of script
use easy_sockets::{
    error::{deserialize_error, serialize_error, ErrorCode},
    logger::log_error,
    sleep, start_client, Deserialize, Duration, Serialize, SimpleClient,
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

/// Client side
struct Client {
    ping_count: usize,
}
impl Client {
    pub fn new() -> Self {
        Self { ping_count: 0 }
    }
}
impl SimpleClient for Client {
    type ClientMsg = ClientMsg;
    type ServerMsg = ServerMsg;

    fn update(&mut self) -> Option<()> {
        self.send_message(ClientMsg::Ping("Hello Server".into()))
            .expect("Failed to send message");
        sleep(Duration::from_secs(1));
        // If you return None, client shuts down.
        Some(())
    }

    fn handle_response(&mut self, response: Self::ServerMsg) {
        match response {
            ServerMsg::Error(error) => log_error(error),
            ServerMsg::Ping(msg) => {
                println!("Ping Received From Server: {}", msg);
                self.ping_count += 1;
            }
        }
    }
}

fn main() {
    start_client("127.0.0.1:8000", Client::new()).expect("Failed to open client");
}
