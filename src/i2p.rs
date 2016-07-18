use nom::IResult;
use parsers::{sam_hello};
use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Error, Read, Write};

static SAM_MIN: &'static str = "3.0";
static SAM_MAX: &'static str = "3.1";

pub struct I2p {
    stream: TcpStream
}

impl I2p {
    fn handshake(&mut self) {
        let hello_msg = format!("HELLO VERSION MIN={min} MAX={max} \n",
                                min = SAM_MIN,
                                max = SAM_MAX);
        let _ = self.stream.write(&hello_msg.into_bytes());

        let mut buffer = String::new();
        let _ = self.stream.read_to_string(&mut buffer);

        let res = sam_hello(&buffer);
        match res {
            IResult::Done(i, o) => println!("i: {} | o: {:?}", i, o),
            _ => println!("error")
        }
    }

    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<I2p, Error> {
        let tcp_stream = try!(TcpStream::connect(addr));
        let mut i2p = I2p { stream: tcp_stream };

        i2p.handshake();

        Ok(i2p)
    }
}
