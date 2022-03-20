use std::fmt::{self, Display};
use std::io;

use failure::{Backtrace, Context, Fail};
use nom;

/// I2P/SAM error definition
#[derive(Debug, Fail)]
pub struct Error {
	inner: Context<ErrorKind>,
}

/// Kinds of I2P/SAM errors
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
	/// Wraps io errors
	#[fail(display = "IO error occurred (is i2p running?): {}", _0)]
	Io(String),
	/// Wraps nom parser errors
	#[fail(display = "Failed to parse an I2P/SAM message")]
	MessageParsing,
	#[fail(display = "Failed to parse an I2P/SAM message")]
	UnresolvableAddress,
	#[fail(display = "Invalid or unrecognized I2P/SAM message: {}", _0)]
	SAMInvalidMessage(String),
	#[fail(display = "Can't reach peer: {}", _0)]
	SAMCantReachPeer(String),
	#[fail(display = "Destination key not found: {}", _0)]
	SAMKeyNotFound(String),
	#[fail(display = "Peer not found: {}", _0)]
	SAMPeerNotFound(String),
	#[fail(display = "Duplicate peer destination: {}", _0)]
	SAMDuplicatedDest(String),
	#[fail(display = "Invalid destination key: {}", _0)]
	SAMInvalidKey(String),
	#[fail(display = "Invalid stream id: {}", _0)]
	SAMInvalidId(String),
	#[fail(display = "I2P/SAM Timeout: {}", _0)]
	SAMTimeout(String),
	#[fail(display = "Unknown I2P/SAM error: {}", _0)]
	SAMI2PError(String),
	#[fail(display = "I2P address isn't a valid b32 or b64 encoding: {}", _0)]
	BadAddressEncoding(String),
}

impl ErrorKind {
	pub fn to_err(self) -> Error {
		Error {
			inner: Context::new(self),
		}
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let cause = match self.cause() {
			Some(c) => format!("{}", c),
			None => String::from("Unknown"),
		};
		let backtrace = match self.backtrace() {
			Some(b) => format!("{}", b),
			None => String::from("Unknown"),
		};
		let output = format!(
			"{} \n Cause: {} \n Backtrace: {}",
			self.inner, cause, backtrace
		);
		Display::fmt(&output, f)
	}
}

impl Error {
	/// get kind
	pub fn kind(&self) -> ErrorKind {
		self.inner.get_context().clone()
	}
	/// get cause
	pub fn cause(&self) -> Option<&dyn Fail> {
		self.inner.cause()
	}
	/// get backtrace
	pub fn backtrace(&self) -> Option<&Backtrace> {
		self.inner.backtrace()
	}
}

impl From<ErrorKind> for Error {
	fn from(kind: ErrorKind) -> Error {
		Error {
			inner: Context::new(kind),
		}
	}
}

impl From<Context<ErrorKind>> for Error {
	fn from(inner: Context<ErrorKind>) -> Error {
		Error { inner: inner }
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error {
			inner: Context::new(ErrorKind::Io(err.to_string())),
		}
	}
}

impl<I, E> From<nom::Err<I, E>> for Error {
	fn from(_err: nom::Err<I, E>) -> Error {
		Error {
			inner: Context::new(ErrorKind::MessageParsing),
		}
	}
}
