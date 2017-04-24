use nom::IResult;
use parsers::{sam_hello, sam_naming_reply, sam_session_status};
use std::clone::Clone;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::net::{TcpStream, ToSocketAddrs};
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
}

pub struct Stream<'b> {
    sam: SamConnection,
    session: &'b Session,
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
        try!(self.conn.write(&msg.into_bytes()));

        let mut reader = BufReader::new(&self.conn);
        let mut buffer = String::new();
        try!(reader.read_line(&mut buffer));

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
    pub fn create(destination: &str, nickname: &str, style: SessionStyle) -> Result<Session, Error> {
        let mut sam = SamConnection::connect("").unwrap();
        let create_session_msg = format!("SESSION CREATE STYLE={style} ID={nickname} DESTINATION={destination} \n",
                                         style = style.string(),
                                         nickname = nickname,
                                         destination = destination);

        try!(sam.send(create_session_msg, sam_session_status));

        Ok(Session { sam: sam })
    }
}

impl<'b> Stream<'b> {}
