use std::cmp::Ordering;
use std::fmt;
use std::hash;

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

impl fmt::Debug for I2pAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl Clone for I2pAddr {
    fn clone(&self) -> I2pAddr {
        I2pAddr::new(&self.inner)
    }
}

impl PartialEq for I2pAddr {
    fn eq(&self, other: &I2pAddr) -> bool {
        self.inner == other.inner
    }
}

impl Eq for I2pAddr {}

impl hash::Hash for I2pAddr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.inner.hash(s)
    }
}

impl PartialOrd for I2pAddr {
    fn partial_cmp(&self, other: &I2pAddr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for I2pAddr {
    fn cmp(&self, other: &I2pAddr) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}
