//! Networking primitives.
//!
//! The types provided in this module are non-blocking by default and are
//! designed to be portable across all supported Mio platforms. As long as the
//! [portability guidelines] are followed, the behavior should be identical no
//! matter the target platform.
//!
//! [portability guidelines]: ../struct.Poll.html#portability

mod tcp;
pub use self::tcp::{TcpListener, TcpSocket, TcpStream, TcpKeepalive};

mod udp;
pub use self::udp::UdpSocket;

#[cfg(any(unix, target_env = "sgx"))]
mod uds;
#[cfg(any(unix, target_env = "sgx"))]
pub use self::uds::{SocketAddr, UnixDatagram, UnixListener, UnixStream};
