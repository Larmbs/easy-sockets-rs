//! Defines client and server communicating over UDP
use super::errors::{SocketError, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::net::{ToSocketAddrs, UdpSocket};
use std::sync::Mutex;

use super::MAX_PAYLOAD_SIZE;

use crate::sendable::Sendable;

lazy_static! {
    static ref CLIENT_UDP_SOCKET: Mutex<Option<UdpSocket>> = Mutex::new(None);
}

/// Trait which defines a client using a simplified set of the UDP protocol
pub trait ClientUDP {
    type ClientMsg: Serialize + for<'de> Deserialize<'de> + Send + 'static;
    type ServerMsg: Serialize + for<'de> Deserialize<'de> + Send + 'static;

    fn send<A: ToSocketAddrs>(&mut self, message: Self::ClientMsg, addr: A) -> Result<()> {
        let bytes = message.to_bytes()
            .map_err(|e| SocketError::MessageError(e.to_string()))?;

        // Check the payload size
        if bytes.len() > MAX_PAYLOAD_SIZE {
            return Err(SocketError::PayloadSizeError { 
                size: bytes.len(), 
                max: MAX_PAYLOAD_SIZE,
            });
        }

        let mut client = CLIENT_UDP_SOCKET.lock().map_err(|_| {
            SocketError::ProtocolError("Failed to acquire lock on UDP socket".to_string())
        })?;

        let stream = client.as_mut().ok_or_else(|| {
            SocketError::ConnectionError("UDP socket not initialized".to_string())
        })?;

        stream.send_to(&bytes, addr).map_err(|e| {
            SocketError::IoError(e)
        })?;


        Ok(())
    }
}
