pub use self::addr::{I2pSocketAddr, ToI2pSocketAddrs};
pub use self::i2p::I2pAddr;

mod addr;
mod i2p;
#[cfg(test)]
mod test;