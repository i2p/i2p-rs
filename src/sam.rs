use std::io::prelude::*;

use std::clone::Clone;
use std::collections::HashMap;
use std::io;
use std::io::{BufReader, Error, ErrorKind};
use std::net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs};

use log::debug;
use nom::IResult;

use crate::parsers::{sam_hello, sam_naming_reply, sam_session_status, sam_stream_status};

pub static DEFAULT_API: &'static str = "127.0.0.1:7656";

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

pub struct StreamConnect {
	sam: SamConnection,
	session: Session,
	peer_dest: String,
	peer_port: u16,
	local_port: u16,
}

impl SessionStyle {
	fn string(&self) -> &str {
		match *self {
			SessionStyle::Datagram => "DATAGRAM",
			SessionStyle::Raw => "RAW",
			SessionStyle::Stream => "STREAM",
		}
	}
}

fn verify_response<'a>(vec: &'a [(&str, &str)]) -> Result<HashMap<&'a str, &'a str>, Error> {
	let new_vec = vec.clone();
	let map: HashMap<&str, &str> = new_vec.iter().map(|&(k, v)| (k, v)).collect();
	let res = map.get("RESULT").unwrap_or(&"OK").clone();
	let msg = map.get("MESSAGE").unwrap_or(&"").clone();
	match res {
		"OK" => Ok(map),
		"CANT_REACH_PEER" | "KEY_NOT_FOUND" | "PEER_NOT_FOUND" => {
			Err(Error::new(ErrorKind::NotFound, msg))
		}
		"DUPLICATED_DEST" => Err(Error::new(ErrorKind::AddrInUse, msg)),
		"INVALID_KEY" | "INVALID_ID" => Err(Error::new(ErrorKind::InvalidInput, msg)),
		"TIMEOUT" => Err(Error::new(ErrorKind::TimedOut, msg)),
		"I2P_ERROR" => Err(Error::new(ErrorKind::Other, msg)),
		_ => Err(Error::new(ErrorKind::Other, msg)),
	}
}

impl SamConnection {
	fn send<F>(&mut self, msg: String, reply_parser: F) -> Result<HashMap<String, String>, Error>
	where
		F: Fn(&str) -> IResult<&str, Vec<(&str, &str)>>,
	{
		debug!("-> {}", &msg);
		self.conn.write_all(&msg.into_bytes())?;

		let mut reader = BufReader::new(&self.conn);
		let mut buffer = String::new();
		reader.read_line(&mut buffer)?;
		debug!("<- {}", &buffer);

		let response = reply_parser(&buffer);
		let vec_opts = response.unwrap().1;
		verify_response(&vec_opts).map(|m| {
			m.iter()
				.map(|(k, v)| (k.to_string(), v.to_string()))
				.collect()
		})
	}

	fn handshake(&mut self) -> Result<HashMap<String, String>, Error> {
		let hello_msg = format!(
			"HELLO VERSION MIN={min} MAX={max} \n",
			min = SAM_MIN,
			max = SAM_MAX
		);
		self.send(hello_msg, sam_hello)
	}

	pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<SamConnection, Error> {
		let tcp_stream = TcpStream::connect(addr)?;

		let mut socket = SamConnection { conn: tcp_stream };

		socket.handshake()?;

		Ok(socket)
	}

	// TODO: Implement a lookup table
	pub fn naming_lookup(&mut self, name: &str) -> Result<String, Error> {
		let create_naming_lookup_msg = format!("NAMING LOOKUP NAME={name} \n", name = name);
		let ret = self.send(create_naming_lookup_msg, sam_naming_reply)?;
		Ok(ret["VALUE"].clone())
	}

	pub fn duplicate(&self) -> io::Result<SamConnection> {
		self.conn.try_clone().map(|s| SamConnection { conn: s })
	}
}

impl Session {
	pub fn create<A: ToSocketAddrs>(
		sam_addr: A,
		destination: &str,
		nickname: &str,
		style: SessionStyle,
	) -> Result<Session, Error> {
		let mut sam = SamConnection::connect(sam_addr).unwrap();
		let create_session_msg = format!(
			"SESSION CREATE STYLE={style} ID={nickname} DESTINATION={destination} \n",
			style = style.string(),
			nickname = nickname,
			destination = destination
		);

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

	pub fn duplicate(&self) -> io::Result<Session> {
		self.sam.duplicate().map(|s| Session {
			sam: s,
			local_dest: self.local_dest.clone(),
		})
	}
}

impl StreamConnect {
	pub fn new<A: ToSocketAddrs>(
		sam_addr: A,
		destination: &str,
		port: u16,
		nickname: &str,
	) -> io::Result<StreamConnect> {
		let mut session = Session::create(sam_addr, "TRANSIENT", nickname, SessionStyle::Stream)?;

		let mut sam = SamConnection::connect(session.sam_api()?).unwrap();
		let dest = sam.naming_lookup(destination);

		let create_stream_msg = format!(
			"STREAM CONNECT ID={nickname} DESTINATION={destination} SILENT=false TO_PORT={port}\n",
			nickname = nickname,
			destination = dest.unwrap(),
			port = port
		);

		sam.send(create_stream_msg, sam_stream_status)?;

		let peer_dest = session.naming_lookup(destination)?;

		Ok(StreamConnect {
			sam: sam,
			session: session,
			peer_dest: peer_dest,
			peer_port: port,
			local_port: 0,
		})
	}

	pub fn peer_addr(&self) -> io::Result<(String, u16)> {
		Ok((self.peer_dest.clone(), self.peer_port))
	}

	pub fn local_addr(&self) -> io::Result<(String, u16)> {
		Ok((self.session.local_dest.clone(), self.local_port))
	}

	pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
		self.sam.conn.shutdown(how)
	}

	pub fn duplicate(&self) -> io::Result<StreamConnect> {
		Ok(StreamConnect {
			sam: self.sam.duplicate()?,
			session: self.session.duplicate()?,
			peer_dest: self.peer_dest.clone(),
			peer_port: self.peer_port,
			local_port: self.local_port,
		})
	}
}

impl Read for StreamConnect {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.sam.conn.read(buf)
	}
}

impl Write for StreamConnect {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.sam.conn.write(buf)
	}
	fn flush(&mut self) -> io::Result<()> {
		self.sam.conn.flush()
	}
}
