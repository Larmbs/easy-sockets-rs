//! Easy Sockets Rust
//! -----------------
//! Quick and easy way of building up complex socket protocols.
//!

use anyhow::{Context, Result};
use lazy_static::lazy_static;
pub use serde::{Deserialize, Serialize};

use std::net::TcpStream;
use std::sync::Mutex;

pub use std::{thread::sleep, time::Duration};
pub use tokio;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net;

pub mod error;
pub mod logger;
pub mod prelude;
pub mod sendable;
pub mod sockets;

use sendable::Sendable;

lazy_static! {
    static ref CLIENT: Mutex<Option<TcpStream>> = Mutex::new(None);
}

/// The default buffer size expected from a socket message.
const BUFFER_SIZE: usize = 1024;
pub type Bytes = Vec<u8>;

/// Trait that simplifies the creation of the server side of a socket protocol.
pub trait ServerConn {
    type ClientMsg: Serialize + for<'de> Deserialize<'de> + Send + 'static;
    type ServerMsg: Serialize + for<'de> Deserialize<'de> + Send + 'static;

    /// Handle Message
    /// --------------
    /// This function runs every time the server-conn receives a client message. You need to decide what message tp respond with.
    ///
    fn handle_message(&mut self, message: Self::ClientMsg) -> Self::ServerMsg;

    /// Opens a new server connection.
    fn new() -> Self;
}

pub async fn start_server<T: ServerConn + Send + 'static>(
    address: impl net::ToSocketAddrs,
) -> Result<()> {
    // Initialize and bind the server
    let listener = net::TcpListener::bind(address)
        .await
        .context("Failed to bind to address")?;

    // Accept and handle connections
    while let Ok((socket, _)) = listener.accept().await {
        tokio::spawn(async move {
            let res = handle_connection(socket, &mut T::new()).await;
            eprintln!("{:?}", res);
        });
    }

    Ok(())
}

async fn handle_connection<T: ServerConn + Send + 'static>(
    mut socket: tokio::net::TcpStream,
    instance: &mut T,
) -> Result<()> {
    let (reader, writer) = socket.split();
    let mut reader = tokio::io::BufReader::new(reader);
    let mut writer = tokio::io::BufWriter::new(writer);

    let mut buf = vec![0; BUFFER_SIZE];
    loop {
        let n = reader
            .read(&mut buf)
            .await
            .context("Failed to read from socket")?;
        if n == 0 {
            break; // Connection closed
        }

        // Ensure buf contains only the data read
        let message = T::ClientMsg::from_bytes(&buf).context("Failed to parse message")?;
        let response = instance.handle_message(message);

        let response_bytes = response
            .to_bytes()
            .context("Failed to serialize response")?;
        writer
            .write_all(&response_bytes)
            .await
            .context("Failed to write to socket")?;
        writer.flush().await.context("Failed to flush writer")?;
    }

    Ok(())
}
