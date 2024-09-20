//! Defines messages able to be sent between devices
use super::Bytes;
use anyhow::{Context, Result};
use bincode::{deserialize, serialize};
pub use serde::{Deserialize, Serialize};

/// Type able to be sent between devices
pub trait Sendable {
    fn to_bytes(&self) -> Result<Bytes>;
    fn from_bytes(bytes: &Bytes) -> Result<Self>
    where
        Self: Sized;
}
impl<T> Sendable for T
where
    T: Serialize + for<'de> Deserialize<'de> + Send + 'static,
{
    /// Converts object to bytes
    fn to_bytes(&self) -> Result<Bytes> {
        serialize(self).context("Failed to serialize object")
    }
    /// Creates an object from bytes
    fn from_bytes(bytes: &Bytes) -> Result<Self> {
        deserialize(bytes).context("Failed to deserialize object")
    }
}
