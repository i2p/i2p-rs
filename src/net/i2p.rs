use std::fmt;

use data_encoding::{Encoding, Specification, BASE32};
use lazy_static::lazy_static;
use log::error;
use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::{Error, ErrorKind};

pub const B32_EXT: &'static str = ".b32.i2p";

lazy_static! {
	static ref BASE32_I2P: Encoding = {
		let mut spec = Specification::new();

		spec.symbols.push_str("abcdefghijklmnopqrstuvwxyz234567");
		spec.padding = None;
		spec.encoding().unwrap()
	};
}

lazy_static! {
	static ref BASE64_I2P: Encoding = {
		let mut spec = Specification::new();
		spec.symbols
			.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-~");
		spec.padding = Some('=');
		spec.encoding().unwrap()
	};
}

/// An I2P address, as a Destination, B32 address or hostname.
///
/// # Examples
///
/// Constructing from a hostname:
///
/// ```
/// use i2p::net::I2pAddr;
///
/// I2pAddr::new("example.i2p");
/// ```
///
/// Constructing from a B32 address:
///
/// ```
/// use i2p::net::I2pAddr;
///
/// I2pAddr::new("abcdefghijklmnopqrstuvwxyz234567abcdefghijklmnopqrst.b32.i2p");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct I2pAddr {
	inner: String,
}

impl I2pAddr {
	/// Creates a new I2p address from a given string.
	///
	/// # Examples
	///
	/// ```
	/// use i2p::net::I2pAddr;
	///
	/// let addr = I2pAddr::new("example.i2p");
	/// ```
	pub fn new(dest: &str) -> I2pAddr {
		I2pAddr {
			inner: dest.to_string(),
		}
	}

	/// Creates a new I2P address from a full base64 destination string. This
	/// will internally convert it to a common base32 addresse, using the
	/// b32.i2p extension.
	pub fn from_b64(dest: &str) -> Result<I2pAddr, Error> {
		let bin_data = BASE64_I2P.decode(dest.as_bytes()).map_err(|e| {
			error!("Base64 decoding error: {:?}", e);
			ErrorKind::BadAddressEncoding(dest.to_string()).to_err()
		})?;
		let mut hasher = Sha256::new();
		hasher.update(bin_data);
		let mut b32 = BASE32_I2P.encode(&hasher.finalize());
		b32.push_str(B32_EXT);
		Ok(I2pAddr { inner: b32 })
	}

	/// Returns the String that makes up this address.
	///
	/// # Examples
	///
	/// ```
	/// use i2p::net::I2pAddr;
	///
	/// let addr = I2pAddr::new("example.i2p");
	/// assert_eq!(addr.string(), "example.i2p");
	/// ```
	pub fn string(&self) -> String {
		self.inner.clone()
	}
}

impl fmt::Display for I2pAddr {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "{}", self.inner)
	}
}
