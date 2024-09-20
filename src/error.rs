//! Defines a trait that represents an error code and its message.

/// Trait definable on enun to define error codes and their messages.
pub trait ErrorCode {
    fn to_code(&self) -> u16;
    fn from_code(code: u16) -> Self;
    fn message(&self) -> &'static str;
}
