use std::io::prelude::*;

use rand;
use rand::Rng;
use std::fmt;
use std::io;
use std::net::{Shutdown, SocketAddr, ToSocketAddrs};

use sam::Stream;

pub use self::addr::{I2pSocketAddr, ToI2pSocketAddrs};
pub use self::i2p::I2pAddr;

mod addr;
mod i2p;
#[cfg(test)]
mod test;

fn each_addr<A: ToSocketAddrs, B: ToI2pSocketAddrs, F, T>(sam_addr: A, addr: B, mut f: F) -> io::Result<T>
where
    F: FnMut(&SocketAddr, &I2pSocketAddr) -> io::Result<T>,
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
    Err(
        last_err.unwrap_or_else(
            || {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "could not resolve to any addresses",
                )
            },
        ),
    )
}

pub struct I2pStream {
    inner: Stream,
}

impl I2pStream {
    pub fn connect<A: ToI2pSocketAddrs>(addr: A) -> io::Result<I2pStream> {
        I2pStream::connect_via("127.0.0.1:7656", addr)
    }

    pub fn connect_via<A: ToSocketAddrs, B: ToI2pSocketAddrs>(sam_addr: A, addr: B) -> io::Result<I2pStream> {
        each_addr(sam_addr, addr, I2pStream::connect_addr)
    }

    fn connect_addr(sam_addr: &SocketAddr, addr: &I2pSocketAddr) -> io::Result<I2pStream> {
        let suffix: String = rand::thread_rng().gen_ascii_chars().take(8).collect();
        let nickname = format!("i2prs-{}", suffix);

        let stream = Stream::new(sam_addr, &addr.dest().string(), addr.port(), &nickname)?;

        Ok(I2pStream { inner: stream })
    }

    pub fn peer_addr(&self) -> io::Result<I2pAddr> {
        self.inner.peer_addr().map(|a| I2pAddr::new(&a))
    }

    pub fn local_addr(&self) -> io::Result<I2pAddr> {
        self.inner.local_addr().map(|a| I2pAddr::new(&a))
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.inner.shutdown(how)
    }

    pub fn try_clone(&self) -> io::Result<I2pStream> {
        self.inner.duplicate().map(|s| I2pStream { inner: s })
    }
}

impl Read for I2pStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for I2pStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl fmt::Debug for I2pStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = f.debug_struct("I2pStream");

        if let Ok(addr) = self.local_addr() {
            res.field("addr", &addr);
        }

        if let Ok(peer) = self.peer_addr() {
            res.field("peer", &peer);
        }

        res.finish()
    }
}
