//! Client side of script
use easy_sockets::{sleep, start_client, Deserialize, Duration, Serialize, SimpleClient};

#[derive(Serialize, Deserialize)]
enum ClientMsg {
    Ping(String),
}

#[derive(Serialize, Deserialize)]
enum ServerMsg {
    Error(u16),
    Ping(String),
}

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
        self.send_message(ClientMsg::Ping("Hello Server".to_string()))
            .expect("Failed to send message");
        sleep(Duration::from_secs(1));
        // If you return None, client shuts down.
        Some(())
    }

    fn handle_response(&mut self, response: Self::ServerMsg) {
        match response {
            ServerMsg::Error(code) => println!("Error Code Received From Server: {}", code),
            ServerMsg::Ping(msg) => {
                println!("Ping Received From Server: {}", msg);
                self.ping_count += 1;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    start_client("127.0.0.1:8000", Client::new()).expect("Failed to open client");
}
