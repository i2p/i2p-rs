pub use self::addr::{I2pSocketAddr, ToI2pSocketAddrs};
pub use self::datagram::I2pDatagramSocket;
pub use self::i2p::I2pAddr;
pub use self::streaming::{I2pListener, I2pStream};
use crate::error::I2PError;
use anyhow::Result;
use std::net::{SocketAddr, ToSocketAddrs};

pub use self::i2p::B32_EXT;
pub use self::i2p::BASE32_I2P;
pub use self::i2p::BASE64_I2P;

mod addr;
mod datagram;
mod i2p;
mod streaming;
#[cfg(test)]
mod test;

fn each_i2p_addr<A: ToSocketAddrs, B: ToI2pSocketAddrs, F, T>(
	sam_addr: A,
	addr: B,
	mut f: F,
) -> Result<T>
where
	F: FnMut(&SocketAddr, &I2pSocketAddr) -> Result<T>,
{
	let mut last_err = None;
	for addr in addr.to_socket_addrs()? {
		for sam_addr in sam_addr.to_socket_addrs()? {
			match f(&sam_addr, &addr) {
				Ok(l) => return Ok(l),
				Err(e) => last_err = Some(e),
			}
		}
	}
	Err(last_err.unwrap_or(I2PError::UnresolvableAddress.into()))
}

fn each_addr<A: ToSocketAddrs, F, T>(sam_addr: A, mut f: F) -> Result<T>
where
	F: FnMut(&SocketAddr) -> Result<T>,
{
	let mut last_err = None;
	for sam_addr in sam_addr.to_socket_addrs()? {
		match f(&sam_addr) {
			Ok(l) => return Ok(l),
			Err(e) => last_err = Some(e),
		}
	}
	Err(last_err.unwrap_or(I2PError::UnresolvableAddress.into()))
}
