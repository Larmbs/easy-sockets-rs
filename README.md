# Easy Sockets
Hi, this is easy sockets, a Rust Crate aimed at simplifying the process of building a TCP messaging protocol, through ergonomic organization system with added helper functions to speed up development.

A major goal of this project is simplicity, meaning using little dependencies and focusing on reliable rock solid systems. This means limiting our dependency count to a single digit number, allowing us to maximize maintainability of this crate in the process.

## Details
This project is similar to a previous project of mine, in this project I aimed to simplify that creation of a server capable of talking with an ESP32 microcontroller. There are many parallels between the two and to add more features this version held. This project stopped pver complications with ownership, now I plan on remedying this by taking on a more functional approach.

[Easy ESP Project](https://github.com/Larmbs/easy_esp)

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

### Server
```Rust
//! Server side of script
use easy_sockets::{Deserialize, Serialize, ServerConn, start_server, tokio};

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
            },
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

```

## Future Plans
There are a number of plans I have of expanding this system, allowing for faster prototyping and deployment.
- Error code macro
    Allows for quickly defining HTML like error codes where an error is represented by and Integer.
- Server central data
    A set of data that all instances of a server can access.
- Derive macros for traits
- Improved message size
