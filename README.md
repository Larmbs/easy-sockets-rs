# Easy Sockets
Hi, this is easy sockets, a Rust Crate aimed at simplifying the process of building a TCP messaging protocol, through ergonomic organization system with added helper functions to speed up development.

A major goal of this project is simplicity, meaning using little dependencies and focusing on reliable rock solid systems. This means limiting our dependency count to a single digit number, allowing us to maximize maintainability of this crate in the process.

## Examples
Here are some examples of the process.

### Client
```Rust
//! Client side of script
use easy_sockets::{sleep, start_client, Deserialize, Duration, Serialize, SimpleClient};

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

fn main() {
    start_client("127.0.0.1:8000", Client::new()).expect("Failed to open client");
}

```