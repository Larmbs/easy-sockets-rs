//! Easy Sockets Rust
//! -----------------
//! Quick and easy way of building up complex socket protocols.
//!

use anyhow::{Context, Result};
use bincode::{deserialize, serialize};
use lazy_static::lazy_static;
pub use serde::{Deserialize, Serialize};

use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Mutex;

pub use std::{thread::sleep, time::Duration};
pub use tokio;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net;

mod sockets;

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

/// Trait that simplifies the creation of the client side of a socket protocol.
pub trait SimpleClient {
    type ClientMsg: Serialize + for<'de> Deserialize<'de> + Send + 'static;
    type ServerMsg: Serialize + for<'de> Deserialize<'de> + Send + 'static;

    fn send_message(&mut self, message: Self::ClientMsg) -> Result<()> {
        let bytes = serialize(&message).context("Failed to serialize message")?;

        let mut client = CLIENT.lock().unwrap();
        let stream = client
            .as_mut()
            .context("You must first start your client before attempting to message.")?;
        stream.write_all(&bytes).context("Failed to send message")?;

        let mut buf = vec![0; BUFFER_SIZE];
        stream.read(&mut buf)?;
        let response = Self::ServerMsg::from_bytes(&buf)?;
        self.handle_response(response);

        Ok(())
    }

    /// Handle Response
    /// ---------------
    /// After sending a message to the server from the update function. You will receive a response from the server and will need to handle it here.
    ///
    fn handle_response(&mut self, response: Self::ServerMsg);

    /// Runs over and over (main loop).
    fn update(&mut self) -> Option<()>;

    /// Starts up the client.
    fn start_up(&mut self) {
        while self.update().is_some() {}
    }
}

/// Type able to be sent between server a client.
pub trait MsgAble {
    fn to_bytes(&self) -> Result<Bytes>;
    fn from_bytes(bytes: &Bytes) -> Result<Self>
    where
        Self: Sized;
}
impl<T> MsgAble for T
where
    T: Serialize + for<'de> Deserialize<'de> + Send + 'static,
{
    /// Converts object to bytes.
    fn to_bytes(&self) -> Result<Bytes> {
        serialize(self).context("Failed to serialize object")
    }
    /// Creates an object from bytes.
    fn from_bytes(bytes: &Bytes) -> Result<Self> {
        deserialize(bytes).context("Failed to deserialize object")
    }
}

/// Starts client socket stream.
pub fn start_client<T: SimpleClient>(address: impl ToSocketAddrs, client: T) -> Result<()> {
    // Connect to the server.
    let stream = TcpStream::connect(address).context("Failed to connect to server")?;

    // Lock and set the global client.
    {
        let mut client_lock = CLIENT.lock().unwrap();
        *client_lock = Some(stream);
    }

    // Run the client.
    let mut client = client;
    client.start_up();

    Ok(())
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
