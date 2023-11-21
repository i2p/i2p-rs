use crate::error::{Error, ErrorKind};
use crate::sam_options::SAMOptions;
use std::net::{SocketAddr, ToSocketAddrs};

pub use self::addr::{I2pSocketAddr, ToI2pSocketAddrs};
pub use self::datagram::I2pDatagramSocket;
pub use self::i2p::I2pAddr;
pub use self::streaming::{I2pListenerBuilder, I2pListener, I2pStream};

mod addr;
mod datagram;
mod i2p;
mod streaming;
#[cfg(test)]
mod test;

fn each_i2p_addr<A: ToSocketAddrs, B: ToI2pSocketAddrs, F, T>(
	sam_addr: A,
	addr: B,
	opts: SAMOptions,
	mut f: F,
) -> Result<T, Error>
where
	F: FnMut(&SocketAddr, &I2pSocketAddr, SAMOptions) -> Result<T, Error>,
{
	let mut last_err = None;
	for addr in addr.to_socket_addrs()? {
		for sam_addr in sam_addr.to_socket_addrs()? {
			match f(&sam_addr, &addr, opts.clone()) {
				Ok(l) => return Ok(l),
				Err(e) => last_err = Some(e),
			}
		}
	}
	Err(last_err.unwrap_or(ErrorKind::UnresolvableAddress.into()))
}

fn each_addr<A: ToSocketAddrs, F, T>(sam_addr: A, opts: SAMOptions, mut f: F) -> Result<T, Error>
where
	F: FnMut(&SocketAddr, SAMOptions) -> Result<T, Error>,
{
	let mut last_err = None;
	for sam_addr in sam_addr.to_socket_addrs()? {
		match f(&sam_addr, opts.clone()) {
			Ok(l) => return Ok(l),
			Err(e) => last_err = Some(e),
		}
	}
	Err(last_err.unwrap_or(ErrorKind::UnresolvableAddress.into()))
}
