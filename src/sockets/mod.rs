//! Module which defines means of send and receiving messages through sockets.

mod tcp;
mod udp;

/// Determines the socket protocol to be used.
pub enum SocketType {
    /// Socket type with limited safety features making it inherently unreliable.
    /// Best to use this if you want to define a fast low level protocol.
    UDP,
    /// Socket type with features like retransmission and message ordering making it most reliable.
    /// Best to use if you don not care much about the specific details.
    TCP,
}
