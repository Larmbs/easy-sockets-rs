use std::io;
use thiserror::Error;


/// Represents errors that occur during socket operations.
/// 
#[derive(Error, Debug)]
pub enum SocketError {
    /// Socket Related errors
    #[error("Failed to establish connection: {0}")]
    ConnectionError(String),

    /// Message serialization and deserialization errors
    #[error("Message format error: {0}")]
    MessageError(String),

    /// Protocol specific errors
    #[error("Protocol error: {0}")]
    ProtocolError(String),

    /// Payload size errors
    #[error("Payload size error: message size {size} exceeds maximum of {max}")]
    PayloadSizeError {
        size: usize,
        max: usize,
    },

    /// Socket config errors
    #[error("Socket configuration error: {0}")]
    ConfigError(String),

    /// IO errors
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    
}

/// Helper type for results with SocketError
pub type Result<T> = std::result::Result<T, SocketError>;


/// Macro for creating custom error codes with associated methods
#[macro_export]
macro_rules! define_error_codes {
    ($(($code:expr, $variant:ident, $message:expr)),* $(,)?) => {
        #[derive(Debug, Clone, Copy)]
        pub enum ErrorCode {
            $(
                $variant = $code,
            )*
        }

        impl ErrorCode {
            pub fn message(&self) -> &'static str {
                match self {
                    $(
                        ErrorCode::$variant => $message,
                    )*
                }
            }

            pub fn code(&self) -> u16 {
                *self as u16
            }
        }
    };
}



// Example usage of the error code macro
define_error_codes! {
    (1000, InvalidProtocol, "Invalid protocol specified"),
    (1001, ConnectionClosed, "Connection closed unexpectedly"),
    (1002, MessageTooLarge, "Message exceeds maximum size"),
    (1003, SerializationFailed, "Failed to serialize message"),
    (1004, DeserializationFailed, "Failed to deserialize message"),
    (1005, HandshakeFailed, "Connection handshake failed"),
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        assert_eq!(ErrorCode::InvalidProtocol.code(), 1000);
        assert_eq!(
            ErrorCode::InvalidProtocol.message(),
            "Invalid protocol specified"
        );
    }

    #[test]
    fn test_socket_error_display() {
        let err = SocketError::PayloadSizeError {
            size: 2000,
            max: 1450,
        };
        assert_eq!(
            err.to_string(),
            "Payload size error: message size 2000 exceeds maximum of 1450"
        );
    }
}