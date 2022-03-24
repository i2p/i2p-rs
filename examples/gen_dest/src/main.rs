extern crate env_logger;
extern crate i2p;
extern crate log;

use log::*;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{thread, time};

use i2p::sam::{SamConnection, DEFAULT_API};

// Run with RUST_LOG=debug to see the action
fn main() {
	env_logger::init();

	let mut sam_conn = SamConnection::connect(DEFAULT_API).unwrap();
	let (pubkey, seckey) = sam_conn.generate_destination().unwrap();
	println!("New public key: {}", pubkey);
	println!("New secret key: {}", seckey);
}
