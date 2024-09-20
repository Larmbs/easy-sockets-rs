//! Module with helpful logging functions
use crate::error::ErrorCode;

/// Logs error message
pub fn log_error<T: ErrorCode>(error: T) {
    eprintln!("{}", error.message())
}
