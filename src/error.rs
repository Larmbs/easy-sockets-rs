//! Defines a trait that represents an error code and its message.
use serde::{de::Error, Deserialize, Deserializer, Serializer};

/// Trait definable on enun to define error codes and their messages.
pub trait ErrorCode
where
    Self: Sized,
{
    /// Converts error to a code for compression
    fn to_code(&self) -> u16;
    /// Maps code to error type
    fn from_code(code: u16) -> Option<Self>;
    /// Returns error message attached to code
    fn message(&self) -> &'static str;
}

// Helper function to serialize any ErrorCode type
pub fn serialize_error<S, E>(error: &E, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    E: ErrorCode,
{
    serializer.serialize_u16(error.to_code())
}

// Helper function to deserialize any ErrorCode type
pub fn deserialize_error<'de, D, E>(deserializer: D) -> Result<E, D::Error>
where
    D: Deserializer<'de>,
    E: ErrorCode,
{
    let code = u16::deserialize(deserializer)?;
    E::from_code(code).ok_or_else(|| D::Error::custom("Invalid error code"))
}
