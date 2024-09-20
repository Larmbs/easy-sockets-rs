//! Defines client and server communicating over TCP
use crate::sendable::Sendable;
use serde::{Deserialize, Serialize};

use super::MAX_PAYLOAD_SIZE;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Mutex;

lazy_static! {
    static ref CLIENT_TCP_SOCKET: Mutex<Option<TcpStream>> = Mutex::new(None);
}

/// Trait which defines a client using the TCP protocol
pub trait ClientTCP {
    type ClientMsg: Serialize + for<'de> Deserialize<'de> + Send + 'static;
    type ServerMsg: Serialize + for<'de> Deserialize<'de> + Send + 'static;

    fn send_message(&mut self, message: Self::ClientMsg) -> Result<()> {
        let bytes = message.to_bytes()?;

        let mut client = CLIENT_TCP_SOCKET.lock().unwrap();
        let stream = client
            .as_mut()
            .context("You must first start your client before attempting to message.")?;
        stream.write_all(&bytes).context("Failed to send message")?;

        let mut buf = vec![0; MAX_PAYLOAD_SIZE];
        stream.read(&mut buf)?;
        let response = Self::ServerMsg::from_bytes(&buf)?;
        self.handle_response(response);

        Ok(())
    }

    fn handle_response(&mut self, response: Self::ServerMsg);

    fn update(&mut self) -> Option<()>;

    fn start_up(&mut self) {
        while self.update().is_some() {}
    }
}

/// Starts client socket stream.
pub fn start_client<T: ClientTCP>(address: impl ToSocketAddrs, client: T) -> Result<()> {
    // Connect to the server.
    let stream = TcpStream::connect(address).context("Failed to connect to server")?;

    // Lock and set the global client.
    {
        let mut client_lock = CLIENT_TCP_SOCKET.lock().unwrap();
        *client_lock = Some(stream);
    }

    // Run the client.
    let mut client = client;
    client.start_up();

    Ok(())
}
