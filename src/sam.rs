use nom::IResult;
use parsers::{sam_hello, sam_naming_reply, sam_session_status, sam_stream_status};
use std::clone::Clone;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::io;
use std::io::{Error, ErrorKind, BufReader};
use std::io::prelude::*;

static SAM_MIN: &'static str = "3.0";
static SAM_MAX: &'static str = "3.1";

pub enum SessionStyle {
    Datagram,
    Raw,
    Stream,
}

pub struct SamConnection {
    conn: TcpStream,
}

pub struct Session {
    sam: SamConnection,
    local_dest: String,
}

pub struct Stream {
    sam: SamConnection,
    session: Session,
    peer_dest: String,
}

impl SessionStyle {
    fn string<'a>(&'a self) -> &'a str {
        match *self {
            SessionStyle::Datagram => "DATAGRAM",
            SessionStyle::Raw => "RAW",
            SessionStyle::Stream => "STREAM",
        }
    }
}

fn verify_response<'a>(vec: &'a Vec<(&str, &str)>) -> Result<HashMap<&'a str, &'a str>, Error> {
    let newVec = vec.clone();
    let map: HashMap<&str, &str> = newVec.iter().map(|&(k, v)| (k, v)).collect();
    let res = map.get("RESULT").unwrap_or(&"OK").clone();
    let msg = map.get("MESSAGE").unwrap_or(&"").clone();
    match res {
        "OK"               => Ok(map),
        "CANT_REACH_PEER"  => Err(Error::new(ErrorKind::NotFound, msg)),
        "DUPLICATED_DEST"  => Err(Error::new(ErrorKind::AddrInUse, msg)),
        "I2P_ERROR"        => Err(Error::new(ErrorKind::Other, msg)),
        "INVALID_KEY"      => Err(Error::new(ErrorKind::InvalidInput, msg)),
        "KEY_NOT_FOUND"    => Err(Error::new(ErrorKind::NotFound, msg)),
        "PEER_NOT_FOUND"   => Err(Error::new(ErrorKind::NotFound, msg)),
        "INVALID_ID"       => Err(Error::new(ErrorKind::InvalidInput, msg)),
        "TIMEOUT"          => Err(Error::new(ErrorKind::TimedOut, msg)),
        _                   => Err(Error::new(ErrorKind::Other, msg)),
    }
}

impl SamConnection {
    fn send<F>(&mut self, msg: String, reply_parser: F) -> Result<HashMap<String, String>, Error>
        where F: Fn(&str) -> IResult<&str, Vec<(&str, &str)>>
    {
        debug!("-> {}", &msg);
        try!(self.conn.write(&msg.into_bytes()));

        let mut reader = BufReader::new(&self.conn);
        let mut buffer = String::new();
        try!(reader.read_line(&mut buffer));
        debug!("<- {}", &buffer);

        let response = reply_parser(&buffer);
        let vec_opts = response.unwrap().1;
        verify_response(&vec_opts).map(|m| {
            m.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
        })
    }

    fn handshake(&mut self) -> Result<HashMap<String, String>, Error> {
        let hello_msg = format!("HELLO VERSION MIN={min} MAX={max} \n",
                                min = SAM_MIN,
                                max = SAM_MAX);
        self.send(hello_msg, sam_hello)
    }

    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<SamConnection, Error> {
        let tcp_stream = try!(TcpStream::connect(addr));

        let mut socket = SamConnection { conn: tcp_stream };

        try!(socket.handshake());

        Ok(socket)
    }

    // TODO: Implement a lookup table
    pub fn naming_lookup(&mut self, name: &str) -> Result<String, Error> {
        let create_naming_lookup_msg = format!("NAMING LOOKUP NAME={name} \n", name = name);
        let ret = try!(self.send(create_naming_lookup_msg, sam_naming_reply));
        Ok(ret.get("VALUE").unwrap().clone())
    }
}

impl Session {
    pub fn create<A: ToSocketAddrs>(sam_addr: A,
                                    destination: &str,
                                    nickname: &str,
                                    style: SessionStyle)
                                    -> Result<Session, Error> {
        let mut sam = SamConnection::connect(sam_addr).unwrap();
        let create_session_msg = format!("SESSION CREATE STYLE={style} ID={nickname} DESTINATION={destination} \n",
                                         style = style.string(),
                                         nickname = nickname,
                                         destination = destination);

        sam.send(create_session_msg, sam_session_status)?;

        let local_dest = sam.naming_lookup("ME")?;

        Ok(Session {
            sam: sam,
            local_dest: local_dest,
        })
    }

    pub fn sam_api(&self) -> io::Result<SocketAddr> {
        self.sam.conn.peer_addr()
    }

    pub fn naming_lookup(&mut self, name: &str) -> io::Result<String> {
        self.sam.naming_lookup(name)
    }
}

impl Stream {
    pub fn new<A: ToSocketAddrs>(sam_addr: A, destination: &str, port: u16, nickname: &str) -> io::Result<Stream> {
        let mut session = Session::create(sam_addr, "TRANSIENT", &nickname, SessionStyle::Stream)?;

        let mut sam = SamConnection::connect(session.sam_api()?).unwrap();
        let create_stream_msg = format!("STREAM CONNECT ID={nickname} DESTINATION={destination} SILENT=false TO_PORT={port}\n",
                                         nickname = nickname,
                                         destination = destination,
                                         port = port);

        sam.send(create_stream_msg, sam_stream_status)?;

        let peer_dest = session.naming_lookup(destination)?;

        Ok(Stream {
               sam: sam,
               session: session,
               peer_dest: peer_dest,
           })
    }

    pub fn peer_addr(&self) -> io::Result<String> {
        Ok(self.peer_dest.clone())
    }

    pub fn local_addr(&self) -> io::Result<String> {
        Ok(self.session.local_dest.clone())
    }
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.sam.conn.read(buf)
    }
}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.sam.conn.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.sam.conn.flush()
    }
}
