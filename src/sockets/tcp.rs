//! Defines client and server communicating over TCP
use super::errors::{SocketError, Result};
use crate::sendable::Sendable;
use serde::{Deserialize, Serialize};

use super::MAX_PAYLOAD_SIZE;
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
        let bytes = message.to_bytes().map_err(|e| SocketError::MessageError(e.to_string()))?;

        // Check the payload size
        if bytes.len() > MAX_PAYLOAD_SIZE {
            return Err(SocketError::PayloadSizeError {
                size: bytes.len(),
                max: MAX_PAYLOAD_SIZE,
            });
        }


        let mut client = CLIENT_TCP_SOCKET.lock().map_err(|_| {
            SocketError::ProtocolError("Failed to acquire lock on TCP socket".to_string())
        })?;


        let stream = client.as_mut().ok_or_else(|| {
            SocketError::ConnectionError("Client not initialized".to_string())
        })?;

        stream.write_all(&bytes).map_err(|e| {
            SocketError::IoError(e)
        })?;

        let mut buf = vec![0; MAX_PAYLOAD_SIZE];
        stream.read(&mut buf).map_err(|e| SocketError::IoError(e))?;


        let response = Self::ServerMsg::from_bytes(&buf)
            .map_err(|e| SocketError::MessageError(e.to_string()))?;

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
    let stream = TcpStream::connect(address)
        .map_err(|e| SocketError::ConnectionError(e.to_string()))?;
    // Lock and set the global client.
    {
        let mut client_lock = CLIENT_TCP_SOCKET.lock().map_err(|_| {
            SocketError::ProtocolError("Failed to acquire lock on TCP socket".to_string())
        })?;
        *client_lock = Some(stream);
    }


    // Run the client.
    let mut client = client;
    client.start_up();

    Ok(())
}
