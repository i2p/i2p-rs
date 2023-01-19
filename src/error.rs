use nom;

use std::io;
use thiserror::Error as ThisError;

/// Kinds of I2P/SAM errors
#[derive(Clone, Eq, PartialEq, Debug, ThisError)]
pub enum I2PError {
	/// Wraps io errors
	#[error("IO error occurred (is i2p running?): {0}")]
	Io(String),
	/// Wraps nom parser errors
	#[error("Failed to parse an I2P/SAM message")]
	MessageParsing,
	#[error("Failed to parse an I2P/SAM message")]
	UnresolvableAddress,
	#[error("Invalid or unrecognized I2P/SAM message: {0}")]
	SAMInvalidMessage(String),
	#[error("Can't reach peer: {0}")]
	SAMCantReachPeer(String),
	#[error("Destination key not found: {0}")]
	SAMKeyNotFound(String),
	#[error("Peer not found: {0}")]
	SAMPeerNotFound(String),
	#[error("Duplicate peer destination: {0}")]
	SAMDuplicatedDest(String),
	#[error("Invalid destination key: {0}")]
	SAMInvalidKey(String),
	#[error("Invalid stream id: {0}")]
	SAMInvalidId(String),
	#[error("I2P/SAM Timeout: {0}")]
	SAMTimeout(String),
	#[error("Unknown I2P/SAM error: {0}")]
	SAMI2PError(String),
	#[error("I2P address isn't a valid b32 or b64 encoding: {0}")]
	BadAddressEncoding(String),
	#[error("Accept encountered error, and session was recreated. try operation again")]
	SessionRecreated,
}

impl From<io::Error> for I2PError {
	fn from(err: io::Error) -> I2PError {
		Self::Io(err.to_string())
	}
}

impl<E> From<nom::Err<E>> for I2PError {
	fn from(_err: nom::Err<E>) -> I2PError {
		Self::MessageParsing
	}
}
