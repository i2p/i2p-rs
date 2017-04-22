use nom::IResult;
use parsers::{sam_hello, sam_naming_lookup, sam_stream_session};
use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Error, BufReader};
use std::io::prelude::*;

static SAM_MIN: &'static str = "3.0";
static SAM_MAX: &'static str = "3.1";

pub enum SessionStyle {
    Datagram,
    Raw,
    Stream,
}

pub struct Socket {
    stream: TcpStream,
}

pub struct Stream<'b> {
    socket: &'b Socket,
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

fn parse_response(response: IResult<&str, Vec<(&str, &str)>>) -> Result<(), Error> {
    let verify_response = |vec: Vec<(&str, &str)>| -> Result<(), Error> {
        for &(key, value) in &vec {
            println!("{}={}", key, value);
            if key == "RESULT" && value == "OK" {
                return Ok(());
            }
        }
        Ok(())
    };

    match response {
        IResult::Done(_, vec) => verify_response(vec),
        _ => panic!("Parser error"),
    }
}

fn read_line_parse<F>(stream: &TcpStream, parser_fn: F) -> Result<(), Error>
    where F: Fn(&str) -> IResult<&str, Vec<(&str, &str)>>
{

    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    try!(reader.read_line(&mut buffer));

    println!("{}", buffer);
    parse_response(parser_fn(&buffer))
}

impl Socket {
    fn handshake(&mut self) -> Result<(), Error> {
        let hello_msg = format!("HELLO VERSION MIN={min} MAX={max} \n",
                                min = SAM_MIN,
                                max = SAM_MAX);
        try!(self.stream.write(&hello_msg.into_bytes()));
        read_line_parse(&self.stream, sam_hello)
    }

    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Socket, Error> {
        let tcp_stream = try!(TcpStream::connect(addr));

        let mut socket = Socket { stream: tcp_stream };

        try!(socket.handshake());

        Ok(socket)
    }

    // TODO: Implement a lookup table
    pub fn naming_lookup<'a>(&'a mut self, name: &str) -> Result<&'a str, Error> {
        let create_naming_lookup_msg = format!("NAMING LOOKUP NAME={name} \n", name = name);
        try!(self.stream.write(&create_naming_lookup_msg.into_bytes()));

        try!(read_line_parse(&self.stream, sam_naming_lookup));

        Ok("Test")
    }

    pub fn create_session<'b>(&'b mut self,
                              destination: &str,
                              nickname: &str,
                              style: SessionStyle)
                              -> Result<Stream<'b>, Error> {
        let create_session_msg = format!("SESSION CREATE STYLE={style} ID={nickname} DESTINATION={destination} \n",
                                         style = style.string(),
                                         nickname = nickname,
                                         destination = destination);

        try!(self.stream.write(&create_session_msg.into_bytes()));

        try!(read_line_parse(&self.stream, sam_stream_session));

        Ok(Stream { socket: self })
    }
}

impl<'b> Stream<'b> {}
