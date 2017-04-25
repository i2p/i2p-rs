use net::{I2pSocketAddr, I2pAddr, ToI2pSocketAddrs};

pub fn isa(a: I2pAddr, p: u16) -> I2pSocketAddr {
    I2pSocketAddr::new(a, p)
}

pub fn tsa<A: ToI2pSocketAddrs>(a: A) -> Result<Vec<I2pSocketAddr>, String> {
    match a.to_socket_addrs() {
        Ok(a) => Ok(a.collect()),
        Err(e) => Err(e.to_string()),
    }
}