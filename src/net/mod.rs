//! Network-related operations.

use crate::imp;

#[cfg(not(feature = "std"))]
mod addr;
#[cfg(not(feature = "std"))]
mod ip;
mod send_recv;
mod socket;
mod socket_addr_any;
#[cfg(not(any(windows, target_os = "wasi")))]
mod socketpair;
#[cfg(windows)]
mod wsa;

pub mod sockopt;

#[cfg(not(windows))]
pub use send_recv::sendto_unix;
pub use send_recv::{recv, recvfrom, send, sendto_v4, sendto_v6, RecvFlags, SendFlags};
pub use socket::{
    accept, accept_with, acceptfrom, acceptfrom_with, bind_v4, bind_v6, connect_v4, connect_v6,
    getpeername, getsockname, listen, shutdown, socket, socket_with, AcceptFlags, AddressFamily,
    Protocol, Shutdown, SocketFlags, SocketType,
};
#[cfg(not(windows))]
pub use socket::{bind_unix, connect_unix};
pub use socket_addr_any::SocketAddrAny;
#[cfg(not(any(windows, target_os = "wasi")))]
pub use socketpair::socketpair;
#[cfg(windows)]
pub use wsa::{wsa_cleanup, wsa_startup};

pub use imp::net::SocketAddrStorage;
#[cfg(not(windows))]
pub use imp::net::SocketAddrUnix;

// Declare the `Ip` and `Socket` address types.
#[cfg(not(feature = "std"))]
pub use addr::{SocketAddr, SocketAddrV4, SocketAddrV6};
#[cfg(not(feature = "std"))]
pub use ip::{IpAddr, Ipv4Addr, Ipv6Addr, Ipv6MulticastScope};
#[cfg(feature = "std")]
pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
