# Easy Sockets

A lightweight, ergonomic Rust crate for building TCP messaging protocols with minimal dependencies.

## Overview

Easy Sockets simplifies the development of TCP-based communication systems by providing an intuitive organization system and helper functions. The crate emphasizes simplicity and reliability while maintaining a minimal dependency footprint for enhanced maintainability. [Crate Easy Sockets](https://crates.io/crates/easy-sockets)

## Key Features

- Simple and intuitive API for TCP client-server communication
- Built-in serialization and deserialization support
- Minimal dependencies for improved maintainability
- Async support using Tokio
- Type-safe message handling
- Easy-to-use trait implementations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
easy_sockets = "0.1.0"
```

## Quick Start

### Creating a Server

```rust
use easy_sockets::{Deserialize, Serialize, ServerConn, start_server, tokio};

#[derive(Serialize, Deserialize)]
enum ClientMsg {
    Ping(String),
}

#[derive(Serialize, Deserialize)]
enum ServerMsg {
    Error(u16),
    Ping(String),
}

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

### Creating a Client

```rust
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

## Project Goals

- **Simplicity**: Maintain a clean, intuitive API that makes TCP communication straightforward
- **Reliability**: Focus on rock-solid systems and proven patterns
- **Maintainability**: Keep dependencies minimal to ensure long-term sustainability
- **Performance**: Optimize message handling and network operations

## Roadmap

The following features are planned for future releases:

- Error code macro system similar to HTTP status codes
- Shared server state accessible across all instances
- Derive macros for common traits
- Optimized message size and handling
- Enhanced documentation and examples

## Background

This project evolved from previous work on ESP32 microcontroller communication systems. The current implementation takes a more functional approach to address ownership complications encountered in earlier versions.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Related Projects

- **Easy ESP** - Previous project focused on ESP32 communication - [Easy ESP Project](https://github.com/Larmbs/easy_esp)
