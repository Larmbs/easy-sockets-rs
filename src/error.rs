//! Defines a trait that represents an error code and its message.
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Trait definable on enun to define error codes and their messages.
pub trait ErrorCode {
    fn to_code(&self) -> u16;
    fn from_code(code: u16) -> Self;
    fn message(&self) -> &'static str;
}

impl<T> Serialize for T
where
    T: ErrorCode,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(self.to_code())
    }
}

impl<'de, T> Deserialize<'de> for T
where
    T: ErrorCode,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = u16::deserialize(deserializer)?;
        T::from_code(code).ok_or_else(|| serde::de::Error::custom("Invalid error code"))
    }
}
