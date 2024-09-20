//! Defines client and server communicating over UDP
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::net::{ToSocketAddrs, UdpSocket};
use std::sync::Mutex;

use crate::sendable::Sendable;

lazy_static! {
    static ref CLIENT_UDP_SOCKET: Mutex<Option<UdpSocket>> = Mutex::new(None);
}

/// Trait which defines a client using a simplified set of the UDP protocol
pub trait ClientUDP {
    type ClientMsg: Serialize + for<'de> Deserialize<'de> + Send + 'static;
    type ServerMsg: Serialize + for<'de> Deserialize<'de> + Send + 'static;

    fn send<A: ToSocketAddrs>(&mut self, message: Self::ClientMsg, addr: A) -> Result<()> {
        let bytes = message.to_bytes()?;

        let mut client = CLIENT_UDP_SOCKET.lock().unwrap();
        let stream = client
            .as_mut()
            .context("You must first start your client before attempting to message.")?;
        stream
            .send_to(&bytes, addr)
            .context("Failed to send message")?;
        Ok(())
    }
}
