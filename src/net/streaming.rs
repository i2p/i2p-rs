use std::io::prelude::*;

use std::fmt;
use std::io;
use std::net::{Shutdown, SocketAddr, ToSocketAddrs};

use rand;
use rand::Rng;

use net::{I2pAddr, I2pSocketAddr, ToI2pSocketAddrs};
use sam::{DEFAULT_API, StreamConnect};

pub struct I2pStream {
    inner: StreamConnect,
}

/// A structure representing a socket server.
///
/// # Examples
///
/// ```no_run
/// use i2p::net::{I2pListener, I2pStream};
///
/// let listener = I2pListener::bind("127.0.0.1:80").unwrap();
///
/// fn handle_client(stream: I2pStream) {
///     // ...
/// }
///
/// // accept connections and process them serially
/// for stream in listener.incoming() {
///     match stream {
///         Ok(stream) => {
///             handle_client(stream);
///         }
///         Err(e) => { /* connection failed */ }
///     }
/// }
/// ```
pub struct I2pListener {}

/// An infinite iterator over the connections from an `I2pListener`.
///
/// This iterator will infinitely yield [`Some`] of the accepted connections. It
/// is equivalent to calling `accept` in a loop.
///
/// This `struct` is created by the [`incoming`] method on [`I2pListener`].
///
/// [`Some`]: ../../std/option/enum.Option.html#variant.Some
/// [`incoming`]: struct.I2pListener.html#method.incoming
/// [`I2pListener`]: struct.I2pListener.html
#[derive(Debug)]
pub struct Incoming<'a> { listener: &'a I2pListener }

impl I2pStream {
    pub fn connect<A: ToI2pSocketAddrs>(addr: A) -> io::Result<I2pStream> {
        I2pStream::connect_via(DEFAULT_API, addr)
    }

    pub fn connect_via<A: ToSocketAddrs, B: ToI2pSocketAddrs>(sam_addr: A, addr: B) -> io::Result<I2pStream> {
        super::each_addr(sam_addr, addr, I2pStream::connect_addr)
    }

    fn connect_addr(sam_addr: &SocketAddr, addr: &I2pSocketAddr) -> io::Result<I2pStream> {
        let suffix: String = rand::thread_rng().gen_ascii_chars().take(8).collect();
        let nickname = format!("i2prs-{}", suffix);

        let stream = StreamConnect::new(sam_addr, &addr.dest().string(), addr.port(), &nickname)?;

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

impl I2pListener {
    /// Creates a new `I2pListener` which will be bound to the specified
    /// address.
    ///
    /// The returned listener is ready for accepting connections.
    ///
    /// Binding with a port number of 0 is equivalent to binding on every port.
    ///
    /// The address type can be any implementor of `ToI2pSocketAddrs` trait. See
    /// its documentation for concrete examples.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use i2p::net::I2pListener;
    ///
    /// let listener = I2pListener::bind("127.0.0.1:80").unwrap();
    /// ```
    pub fn bind<A: ToI2pSocketAddrs>(addr: A) -> io::Result<I2pListener> {
        I2pListener::bind_via(DEFAULT_API, addr)
    }

    pub fn bind_via<A: ToSocketAddrs, B: ToI2pSocketAddrs>(sam_addr: A, addr: B) -> io::Result<I2pListener> {
        super::each_addr(sam_addr, addr, I2pListener::bind_addr)
    }

    fn bind_addr(sam_addr: &SocketAddr, addr: &I2pSocketAddr) -> io::Result<I2pListener> {
        unimplemented!();
    }

    /// Returns the local socket address of this listener.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use i2p::net::{I2pAddr, I2pSocketAddr, I2pListener};
    ///
    /// let listener = I2pListener::bind("127.0.0.1:8080").unwrap();
    /// assert_eq!(listener.local_addr().unwrap(),
    ///            I2pSocketAddr::new(I2pAddr::new("example.i2p"), 8080));
    /// ```
    pub fn local_addr(&self) -> io::Result<I2pSocketAddr> {
        unimplemented!()
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// The returned `TcpListener` is a reference to the same socket that this
    /// object references. Both handles can be used to accept incoming
    /// connections and options set on one listener will affect the other.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use i2p::net::I2pListener;
    ///
    /// let listener = I2pListener::bind("127.0.0.1:8080").unwrap();
    /// let listener_clone = listener.try_clone().unwrap();
    /// ```
    pub fn try_clone(&self) -> io::Result<I2pListener> {
        unimplemented!()
    }

    /// Accept a new incoming connection from this listener.
    ///
    /// This function will block the calling thread until a new TCP connection
    /// is established. When established, the corresponding `TcpStream` and the
    /// remote peer's address will be returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use i2p::net::I2pListener;
    ///
    /// let listener = I2pListener::bind("127.0.0.1:8080").unwrap();
    /// match listener.accept() {
    ///     Ok((_socket, addr)) => println!("new client: {:?}", addr),
    ///     Err(e) => println!("couldn't get client: {:?}", e),
    /// }
    /// ```
    pub fn accept(&self) -> io::Result<(I2pStream, I2pSocketAddr)> {
        unimplemented!()
    }

    /// Returns an iterator over the connections being received on this
    /// listener.
    ///
    /// The returned iterator will never return [`None`] and will also not yield
    /// the peer's [`I2pSocketAddr`] structure.
    ///
    /// [`None`]: ../../std/option/enum.Option.html#variant.None
    /// [`I2pSocketAddr`]: ../../std/net/struct.I2pSocketAddr.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use i2p::net::I2pListener;
    ///
    /// let listener = I2pListener::bind("127.0.0.1:80").unwrap();
    ///
    /// for stream in listener.incoming() {
    ///     match stream {
    ///         Ok(stream) => {
    ///             println!("new client!");
    ///         }
    ///         Err(e) => { /* connection failed */ }
    ///     }
    /// }
    /// ```
    pub fn incoming(&self) -> Incoming {
        Incoming { listener: self }
    }
}

impl<'a> Iterator for Incoming<'a> {
    type Item = io::Result<I2pStream>;
    fn next(&mut self) -> Option<io::Result<I2pStream>> {
        Some(self.listener.accept().map(|p| p.0))
    }
}

impl fmt::Debug for I2pListener {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}
