use std::net::{SocketAddr, ToSocketAddrs};

use crate::error::{Error, ErrorKind};
use crate::net::{I2pSocketAddr, ToI2pSocketAddrs};
use crate::sam::DEFAULT_API;

/// Unimplemented
///
/// An I2P datagram socket.
///
/// This is an implementation of a bound datagram socket. There is no
/// corresponding notion of a server because is a datagram protocol.
///
/// # Examples
///
/// ```no_run
/// use i2p::net::I2pDatagramSocket;
/// use i2p::Error;
///
/// # fn foo() -> Result<(), Error> {
/// {
///     let mut socket = I2pDatagramSocket::bind("127.0.0.1:34254")?;
///
///     // read from the socket
///     let mut buf = [0; 10];
///     let (amt, src) = socket.recv_from(&mut buf)?;
///
///     // send a reply to the socket we received data from
///     let buf = &mut buf[..amt];
///     buf.reverse();
///     socket.send_to(buf, &src)?;
///     # Ok(())
/// } // the socket is closed here
/// # }
/// ```
pub struct I2pDatagramSocket {}

impl I2pDatagramSocket {
	/// Creates an I2P datagram socket from the given address.
	///
	/// The address type can be any implementor of [`ToI2pSocketAddrs`] trait. See
	/// its documentation for concrete examples.
	///
	/// [`ToI2pSocketAddrs`]: ../../i2p/net/trait.ToI2pSocketAddrs.html
	///
	/// # Examples
	///
	/// ```no_run
	/// use i2p::net::I2pDatagramSocket;
	///
	/// let socket = I2pDatagramSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
	/// ```
	pub fn bind<A: ToI2pSocketAddrs>(addr: A) -> Result<I2pDatagramSocket, Error> {
		I2pDatagramSocket::bind_via(DEFAULT_API, addr)
	}

	pub fn bind_via<A: ToSocketAddrs, B: ToI2pSocketAddrs>(
		sam_addr: A,
		addr: B,
	) -> Result<I2pDatagramSocket, Error> {
		super::each_i2p_addr(sam_addr, addr, I2pDatagramSocket::bind_addr).map_err(|e| e.into())
	}

	fn bind_addr(
		_sam_addr: &SocketAddr,
		_addr: &I2pSocketAddr,
	) -> Result<I2pDatagramSocket, Error> {
		unimplemented!();
	}

	/// Receives data from the socket. On success, returns the number of bytes
	/// read and the address from whence the data came.
	///
	/// # Examples
	///
	/// ```no_run
	/// use i2p::net::I2pDatagramSocket;
	///
	/// let socket = I2pDatagramSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
	/// let mut buf = [0; 10];
	/// let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)
	///                                         .expect("Didn't receive data");
	/// ```
	pub fn recv_from(&self, _buf: &mut [u8]) -> Result<(usize, I2pSocketAddr), Error> {
		unimplemented!()
	}

	/// Receives data from the socket, without removing it from the queue.
	///
	/// Successive calls return the same data.
	///
	/// On success, returns the number of bytes peeked and the address from
	/// whence the data came.
	///
	/// # Examples
	///
	/// ```no_run
	/// use i2p::net::I2pDatagramSocket;
	///
	/// let socket = I2pDatagramSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
	/// let mut buf = [0; 10];
	/// let (number_of_bytes, src_addr) = socket.peek_from(&mut buf)
	///                                         .expect("Didn't receive data");
	/// ```
	pub fn peek_from(&self, _buf: &mut [u8]) -> Result<(usize, I2pSocketAddr), Error> {
		unimplemented!()
	}

	/// Sends data on the socket to the given address. On success, returns the
	/// number of bytes written.
	///
	/// Address type can be any implementor of [`ToI2pSocketAddrs`] trait. See
	/// its documentation for concrete examples.
	///
	/// [`ToI2pSocketAddrs`]: ../../std/net/trait.ToI2pSocketAddrs.html
	///
	/// # Examples
	///
	/// ```no_run
	/// use i2p::net::I2pDatagramSocket;
	///
	/// let socket = I2pDatagramSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
	/// socket.send_to(&[0; 10], "127.0.0.1:4242").expect("couldn't send data");
	/// ```
	pub fn send_to<A: ToI2pSocketAddrs>(&self, _buf: &[u8], addr: A) -> Result<usize, Error> {
		match addr.to_socket_addrs()?.next() {
			Some(_addr) => unimplemented!(),
			None => Err(ErrorKind::UnresolvableAddress.into()),
		}
	}

	/// Returns the socket address that this socket was created from.
	///
	/// # Examples
	///
	/// ```no_run
	/// use i2p::net::{I2pAddr, I2pSocketAddr, I2pDatagramSocket};
	///
	/// let socket = I2pDatagramSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
	/// assert_eq!(socket.local_addr().unwrap(),
	///            I2pSocketAddr::new(I2pAddr::new("example.i2p"), 34254));
	/// ```
	pub fn local_addr(&self) -> Result<I2pSocketAddr, Error> {
		unimplemented!()
	}

	/// Creates a new independently owned handle to the underlying socket.
	///
	/// The returned `I2pDatagramSocket` is a reference to the same socket that this
	/// object references. Both handles will read and write the same port, and
	/// options set on one socket will be propagated to the other.
	///
	/// # Examples
	///
	/// ```no_run
	/// use i2p::net::I2pDatagramSocket;
	///
	/// let socket = I2pDatagramSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
	/// let socket_clone = socket.try_clone().expect("couldn't clone the socket");
	/// ```
	pub fn try_clone(&self) -> Result<I2pDatagramSocket, Error> {
		unimplemented!()
	}

	/// Connects this datagram socket to a remote address, allowing the `send` and
	/// `recv` calls to be used to send data and also applies filters to only
	/// receive data from the specified address.
	///
	/// # Examples
	///
	/// ```no_run
	/// use i2p::net::I2pDatagramSocket;
	///
	/// let socket = I2pDatagramSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
	/// socket.connect("127.0.0.1:8080").expect("connect function failed");
	/// ```
	pub fn connect<A: ToI2pSocketAddrs>(&self, addr: A) -> Result<(), Error> {
		self.connect_via(DEFAULT_API, addr)
	}

	pub fn connect_via<A: ToSocketAddrs, B: ToI2pSocketAddrs>(
		&self,
		sam_addr: A,
		addr: B,
	) -> Result<(), Error> {
		super::each_i2p_addr(sam_addr, addr, |_sam_addr, _addr| unimplemented!())
	}

	/// Sends data on the socket to the remote address to which it is connected.
	///
	/// The [`connect()`] method will connect this socket to a remote address. This
	/// method will fail if the socket is not connected.
	///
	/// [`connect()`]: #method.connect
	///
	/// # Examples
	///
	/// ```no_run
	/// use i2p::net::I2pDatagramSocket;
	///
	/// let socket = I2pDatagramSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
	/// socket.connect("127.0.0.1:8080").expect("connect function failed");
	/// socket.send(&[0, 1, 2]).expect("couldn't send message");
	/// ```
	pub fn send(&self, _buf: &[u8]) -> Result<usize, Error> {
		unimplemented!()
	}

	/// Receives data on the socket from the remote address to which it is
	/// connected.
	///
	/// The `connect` method will connect this socket to a remote address. This
	/// method will fail if the socket is not connected.
	///
	/// # Examples
	///
	/// ```no_run
	/// use i2p::net::I2pDatagramSocket;
	///
	/// let socket = I2pDatagramSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
	/// socket.connect("127.0.0.1:8080").expect("connect function failed");
	/// let mut buf = [0; 10];
	/// match socket.recv(&mut buf) {
	///     Ok(received) => println!("received {} bytes", received),
	///     Err(e) => println!("recv function failed: {:?}", e),
	/// }
	/// ```
	pub fn recv(&self, _buf: &mut [u8]) -> Result<usize, Error> {
		unimplemented!()
	}

	/// Receives data on the socket from the remote adress to which it is
	/// connected, without removing that data from the queue. On success,
	/// returns the number of bytes peeked.
	///
	/// Successive calls return the same data.
	///
	/// # Errors
	///
	/// This method will fail if the socket is not connected. The `connect` method
	/// will connect this socket to a remote address.
	///
	/// # Examples
	///
	/// ```no_run
	/// use i2p::net::I2pDatagramSocket;
	///
	/// let socket = I2pDatagramSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
	/// socket.connect("127.0.0.1:8080").expect("connect function failed");
	/// let mut buf = [0; 10];
	/// match socket.peek(&mut buf) {
	///     Ok(received) => println!("received {} bytes", received),
	///     Err(e) => println!("peek function failed: {:?}", e),
	/// }
	/// ```
	pub fn peek(&self, _buf: &mut [u8]) -> Result<usize, Error> {
		unimplemented!()
	}
}
