//! Module which defines means of send and receiving messages through sockets.

pub mod raw_udp;
pub mod tcp;
pub mod udp;

/// Largest single message payload
pub const MAX_PAYLOAD_SIZE: usize = 1450;

/// Determines the socket protocol to be used.
pub enum SocketType {
    /// Socket type with features like retransmission and message ordering making it most reliable.
    /// Best to use if you don not care much about the specific details.
    TCP,
    /// Socket type with limited safety features making it inherently unreliable.
    /// Best to use this if you want to define a fast low level protocol.
    UDP,
    /// Socket type with limited safety but with a wider range of function special to UDP.
    /// Only use this if you need features like multi casting and high flexibility.
    RawUDP,
}
