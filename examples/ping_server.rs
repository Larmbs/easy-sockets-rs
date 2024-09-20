//! Server side of script
use easy_sockets::{start_server, tokio, Deserialize, Serialize, ServerConn};

/// Message that a Client sends
#[derive(Serialize, Deserialize)]
enum ClientMsg {
    Ping(String),
}

/// Message that the server sends
#[derive(Serialize, Deserialize)]
enum ServerMsg {
    Error(u16),
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
            response: "Hello Client".to_string(),
        }
    }
}

#[tokio::main]
async fn main() {
    let _ = start_server::<ServerInstance>("127.0.0.1:8000").await;
}
