use std::io::prelude::*;

use std::clone::Clone;
use std::collections::HashMap;
use std::io::{self, BufReader};
use std::net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs};

use log::debug;
use nom::IResult;

use crate::error::{Error, ErrorKind};
use crate::net::ToI2pSocketAddrs;
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
		"CANT_REACH_PEER" => Err(ErrorKind::SAMCantReachPeer(msg.to_string()).into()),
		"KEY_NOT_FOUND" => Err(ErrorKind::SAMKeyNotFound(msg.to_string()).into()),
		"PEER_NOT_FOUND" => Err(ErrorKind::SAMPeerNotFound(msg.to_string()).into()),
		"DUPLICATED_DEST" => Err(ErrorKind::SAMDuplicatedDest(msg.to_string()).into()),
		"INVALID_KEY" => Err(ErrorKind::SAMInvalidKey(msg.to_string()).into()),
		"INVALID_ID" => Err(ErrorKind::SAMInvalidId(msg.to_string()).into()),
		"TIMEOUT" => Err(ErrorKind::SAMTimeout(msg.to_string()).into()),
		"I2P_ERROR" => Err(ErrorKind::SAMI2PError(msg.to_string()).into()),
		_ => Err(ErrorKind::SAMInvalidMessage(msg.to_string()).into()),
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

		let vec_opts = reply_parser(&buffer)?.1;
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

	pub fn duplicate(&self) -> Result<SamConnection, Error> {
		self.conn.try_clone().map(|s| SamConnection { conn: s }).map_err(|e| e.into())
	}
}

impl Session {
	pub fn create<A: ToSocketAddrs>(
		sam_addr: A,
		destination: &str,
		nickname: &str,
		style: SessionStyle,
	) -> Result<Session, Error> {
		let mut sam = SamConnection::connect(sam_addr)?;
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

	pub fn sam_api(&self) -> Result<SocketAddr, Error> {
		self.sam.conn.peer_addr().map_err(|e| e.into())
	}

	pub fn naming_lookup(&mut self, name: &str) -> Result<String, Error> {
		self.sam.naming_lookup(name)
	}

	pub fn duplicate(&self) -> Result<Session, Error> {
		self.sam.duplicate().map(|s| Session {
			sam: s,
			local_dest: self.local_dest.clone(),
		}).map_err(|e| e.into())
	}
}

impl StreamConnect {
	pub fn new<A: ToSocketAddrs>(
		sam_addr: A,
		destination: &str,
		port: u16,
		nickname: &str,
	) -> Result<StreamConnect, Error> {
		let session = Session::create(sam_addr, "TRANSIENT", nickname, SessionStyle::Stream)?;

		let mut sam = SamConnection::connect(session.sam_api()?).unwrap();
		let dest = sam.naming_lookup(destination)?;

		let create_stream_msg = format!(
			"STREAM CONNECT ID={nickname} DESTINATION={destination} SILENT=false TO_PORT={port}\n",
			nickname = nickname,
			destination = dest,
			port = port
		);

		sam.send(create_stream_msg, sam_stream_status)?;

		Ok(StreamConnect {
			sam: sam,
			session: session,
			peer_dest: dest,
			peer_port: port,
			local_port: 0,
		})
	}

	pub fn peer_addr(&self) -> Result<(String, u16), Error> {
		Ok((self.peer_dest.clone(), self.peer_port))
	}

	pub fn local_addr(&self) -> Result<(String, u16), Error> {
		Ok((self.session.local_dest.clone(), self.local_port))
	}

	pub fn shutdown(&self, how: Shutdown) -> Result<(), Error> {
		self.sam.conn.shutdown(how).map_err(|e| e.into())
	}

	pub fn duplicate(&self) -> Result<StreamConnect, Error> {
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

pub struct StreamForward {
	session: Session,
	local_port: u16,
}

impl StreamForward {
	pub fn new<A: ToSocketAddrs>(
		sam_addr: A,
		bound_port: u16,
		nickname: &str,
	) -> Result<StreamForward, Error> {
		let session = Session::create(sam_addr, "TRANSIENT", nickname, SessionStyle::Stream)?;
		let mut sam = SamConnection::connect(session.sam_api()?).unwrap();

		let create_stream_msg = format!(
			"STREAM FORWARD ID={nickname} PORT={port} SILENT=false\n",
			nickname = nickname,
			port = bound_port,
		);
		sam.send(create_stream_msg, sam_stream_status)?;

		Ok(StreamForward {
			session: session,
			local_port: bound_port,
		})
	}

	pub fn accept(&self, destination: &str, stream: TcpStream) -> Result<StreamConnect, Error> {
		let sam_conn = SamConnection {conn: stream};
		let stream = StreamConnect {
			sam: sam_conn,
			session: self.session.duplicate()?,
			peer_dest: destination.to_string(),
			// port only provided with SAM v3.2+ (not on i2pd)
			peer_port: 0,
			local_port: self.local_port,
		};
		// TODO I2pAddr shouldn't hold the destination directly, but the b32 addr
		Ok(stream)
	}

	pub fn local_addr(&self) -> Result<(String, u16), Error> {
		Ok((self.session.local_dest.clone(), self.local_port))
	}

	pub fn duplicate(&self) -> Result<StreamForward, Error> {
		Ok(StreamForward {
			session: self.session.duplicate()?,
			local_port: self.local_port,
		})
	}
}
