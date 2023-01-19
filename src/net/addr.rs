use std::fmt;
use std::io;
use std::iter;
use std::option;
use std::slice;
use std::vec;

use serde::{Deserialize, Serialize};

use crate::net::i2p::I2pAddr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct I2pSocketAddr {
	port: u16,
	dest: I2pAddr,
}

impl I2pSocketAddr {
	/// Creates a new socket address from the (dest, port) pair.
	///
	/// # Examples
	///
	/// ```
	/// use i2p::net::{I2pAddr, I2pSocketAddr};
	///
	/// let socket = I2pSocketAddr::new(I2pAddr::new("example.i2p"), 8080);
	/// assert_eq!(socket.dest(), I2pAddr::new("example.i2p"));
	/// assert_eq!(socket.port(), 8080);
	/// ```
	pub fn new(dest: I2pAddr, port: u16) -> I2pSocketAddr {
		I2pSocketAddr {
			port: port,
			dest: dest,
		}
	}

	/// Returns the I2P address associated with this socket address.
	///
	/// # Examples
	///
	/// ```
	/// use i2p::net::{I2pAddr, I2pSocketAddr};
	///
	/// let socket = I2pSocketAddr::new(I2pAddr::new("example.i2p"), 8080);
	/// assert_eq!(socket.dest(), I2pAddr::new("example.i2p"));
	/// ```
	pub fn dest(&self) -> I2pAddr {
		self.dest.clone()
	}

	/// Change the I2P address associated with this socket address.
	///
	/// # Examples
	///
	/// ```
	/// use i2p::net::{I2pAddr, I2pSocketAddr};
	///
	/// let mut socket = I2pSocketAddr::new(I2pAddr::new("example.i2p"), 8080);
	/// socket.set_dest(I2pAddr::new("foobar.i2p"));
	/// assert_eq!(socket.dest(), I2pAddr::new("foobar.i2p"));
	/// ```
	pub fn set_dest(&mut self, new_dest: I2pAddr) {
		self.dest = new_dest;
	}

	/// Returns the port number associated with this socket address.
	///
	/// # Examples
	///
	/// ```
	/// use i2p::net::{I2pAddr, I2pSocketAddr};
	///
	/// let socket = I2pSocketAddr::new(I2pAddr::new("example.i2p"), 8080);
	/// assert_eq!(socket.port(), 8080);
	/// ```
	pub fn port(&self) -> u16 {
		self.port
	}

	/// Change the port number associated with this socket address.
	///
	/// # Examples
	///
	/// ```
	/// use i2p::net::{I2pAddr, I2pSocketAddr};
	///
	/// let mut socket = I2pSocketAddr::new(I2pAddr::new("example.i2p"), 8080);
	/// socket.set_port(1025);
	/// assert_eq!(socket.port(), 1025);
	/// ```
	pub fn set_port(&mut self, new_port: u16) {
		self.port = new_port;
	}
}

impl fmt::Display for I2pSocketAddr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}:{}", self.dest(), self.port())
	}
}

/// A trait for objects which can be converted or resolved to one or more
/// `I2pSocketAddr` values.
///
/// This trait is used for generic address resolution when constructing network
/// objects.  By default it is implemented for the following types:
///
///  * `I2pSocketAddr` - `to_socket_addrs` is identity function.
///
///  * `(I2pAddr, u16)` - `to_socket_addrs` constructs `I2pSocketAddr` trivially.
///
///  * `(&str, u16)` - the string should be either a string representation of an
///    I2P address expected by `FromStr` implementation for `I2pAddr` or a host
///    name.
///
///  * `&str` - the string should be either a string representation of a
///    `I2pSocketAddr` as expected by its `FromStr` implementation or a string like
///    `<host_name>:<port>` pair where `<port>` is a `u16` value.
///
/// This trait allows constructing network objects like `I2PStream` or
/// `I2PDatagramSocket` easily with values of various types for the bind/connection
/// address. It is needed because sometimes one type is more appropriate than
/// the other: for simple uses a string like `"example.i2p:12345"` is much nicer
/// than manual construction of the corresponding `I2pSocketAddr`, but sometimes
/// `I2pSocketAddr` value is *the* main source of the address, and converting it to
/// some other type (e.g. a string) just for it to be converted back to
/// `I2pSocketAddr` in constructor methods is pointless.
///
/// Addresses returned by the operating system that are not IP addresses are
/// silently ignored.
///
/// Some examples:
///
/// ```no_run
/// use i2p::net::{I2pSocketAddr, I2pStream, I2pDatagramSocket, I2pListener, I2pAddr};
///
/// fn main() {
///     let dest = I2pAddr::new("example.i2p");
///     let port = 12345;
///
///     // The following lines are equivalent
///     let i2p_s = I2pStream::connect(I2pSocketAddr::new(dest.clone(), port));
///     let i2p_s = I2pStream::connect((dest.clone(), port));
///     let i2p_s = I2pStream::connect(("example.i2p", port));
///     let i2p_s = I2pStream::connect("example.i2p:12345");
///
///     // I2pListener::bind(), I2pDatagramSocket::bind() and I2pDatagramSocket::send_to()
///     // behave similarly
///     let i2p_l = I2pListener::bind();
///
///     let mut i2p_dg_s = I2pDatagramSocket::bind(("127.0.0.1", port)).unwrap();
///     i2p_dg_s.send_to(&[7], (dest, 23451)).unwrap();
/// }
/// ```
pub trait ToI2pSocketAddrs {
	/// Returned iterator over socket addresses which this type may correspond
	/// to.
	type Iter: Iterator<Item = I2pSocketAddr>;

	/// Converts this object to an iterator of resolved `I2pSocketAddr`s.
	///
	/// The returned iterator may not actually yield any values depending on the
	/// outcome of any resolution performed.
	///
	/// Note that this function may block the current thread while resolution is
	/// performed.
	///
	/// # Errors
	///
	/// Any errors encountered during resolution will be returned as an `Err`.
	fn to_socket_addrs(&self) -> io::Result<Self::Iter>;
}

impl ToI2pSocketAddrs for I2pSocketAddr {
	type Iter = option::IntoIter<I2pSocketAddr>;
	fn to_socket_addrs(&self) -> io::Result<option::IntoIter<I2pSocketAddr>> {
		Ok(Some(self.clone()).into_iter())
	}
}

impl ToI2pSocketAddrs for (I2pAddr, u16) {
	type Iter = option::IntoIter<I2pSocketAddr>;
	fn to_socket_addrs(&self) -> io::Result<option::IntoIter<I2pSocketAddr>> {
		let (dest, port) = self.clone();
		I2pSocketAddr::new(dest, port).to_socket_addrs()
	}
}

impl<'a> ToI2pSocketAddrs for (&'a str, u16) {
	type Iter = vec::IntoIter<I2pSocketAddr>;
	fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<I2pSocketAddr>> {
		let (host, port) = *self;
		let addr = I2pSocketAddr::new(I2pAddr::new(host), port);
		Ok(vec![addr].into_iter())
	}
}

// accepts strings like 'example.i2p:12345'
impl ToI2pSocketAddrs for str {
	type Iter = vec::IntoIter<I2pSocketAddr>;
	fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<I2pSocketAddr>> {
		macro_rules! try_opt {
			($e:expr, $msg:expr) => {
				match $e {
					Some(r) => r,
					None => return Err(io::Error::new(io::ErrorKind::InvalidInput, $msg)),
				}
			};
		}

		// split the string by ':' and convert the second part to u16
		let mut parts_iter = self.rsplitn(2, ':');
		let port_str = try_opt!(parts_iter.next(), "invalid I2P socket address");
		let host = try_opt!(parts_iter.next(), "invalid I2P socket address");
		let port: u16 = try_opt!(port_str.parse().ok(), "invalid port value");
		(host, port).to_socket_addrs()
	}
}

impl<'a> ToI2pSocketAddrs for &'a [I2pSocketAddr] {
	type Iter = iter::Cloned<slice::Iter<'a, I2pSocketAddr>>;

	fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
		Ok(self.iter().cloned())
	}
}

impl<'a, T: ToI2pSocketAddrs + ?Sized> ToI2pSocketAddrs for &'a T {
	type Iter = T::Iter;
	fn to_socket_addrs(&self) -> io::Result<T::Iter> {
		(**self).to_socket_addrs()
	}
}

impl ToI2pSocketAddrs for String {
	type Iter = vec::IntoIter<I2pSocketAddr>;
	fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<I2pSocketAddr>> {
		(&**self).to_socket_addrs()
	}
}

#[cfg(test)]
mod tests {
	use crate::net::test::{isa, tsa};
	use crate::net::*;

	#[test]
	fn to_socket_addr_i2paddr_u16() {
		let a = I2pAddr::new("example.i2p");
		let p = 12345;
		let e = I2pSocketAddr::new(a.clone(), p);
		assert_eq!(Ok(vec![e]), tsa((a, p)));
	}

	#[test]
	fn to_socket_addr_str_u16() {
		let a = isa(I2pAddr::new("example.i2p"), 24352);
		assert_eq!(Ok(vec![a]), tsa(("example.i2p", 24352)));

		let a = isa(I2pAddr::new("example.i2p"), 23924);
		assert!(tsa(("example.i2p", 23924)).unwrap().contains(&a));
	}

	#[test]
	fn to_socket_addr_str() {
		let a = isa(I2pAddr::new("example.i2p"), 24352);
		assert_eq!(Ok(vec![a]), tsa("example.i2p:24352"));

		let a = isa(I2pAddr::new("example.i2p"), 23924);
		assert!(tsa("example.i2p:23924").unwrap().contains(&a));
	}

	#[test]
	fn to_socket_addr_string() {
		let a = isa(I2pAddr::new("example.i2p"), 24352);
		assert_eq!(
			Ok(vec![a.clone()]),
			tsa(&*format!("{}:{}", "example.i2p", "24352"))
		);
		assert_eq!(
			Ok(vec![a.clone()]),
			tsa(&format!("{}:{}", "example.i2p", "24352"))
		);
		assert_eq!(
			Ok(vec![a.clone()]),
			tsa(format!("{}:{}", "example.i2p", "24352"))
		);

		let s = format!("{}:{}", "example.i2p", "24352");
		assert_eq!(Ok(vec![a]), tsa(s));
		// s has been moved into the tsa call
	}

	#[test]
	fn set_dest() {
		fn i2p(low: u8) -> I2pAddr {
			I2pAddr::new(&format!("example{}.i2p", low))
		}

		let mut addr = I2pSocketAddr::new(i2p(12), 80);
		assert_eq!(addr.dest(), i2p(12));
		addr.set_dest(i2p(13));
		assert_eq!(addr.dest(), i2p(13));
	}

	#[test]
	fn set_port() {
		let mut addr = I2pSocketAddr::new(I2pAddr::new("example.i2p"), 80);
		assert_eq!(addr.port(), 80);
		addr.set_port(8080);
		assert_eq!(addr.port(), 8080);
	}
}
