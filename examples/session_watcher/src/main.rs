use crossbeam::sync::WaitGroup;
use env_logger;
use i2p;

use crossbeam_channel::select;
use i2p::net::{I2pListener, I2pStream};
use i2p::sam_options::{
	I2CPClientOptions, I2CPOptions, I2CPRouterOptions, I2CPTunnelInboundOptions,
	I2CPTunnelOutboundOptions, SAMOptions, SignatureType,
};
use log::*;
use std::io::{Read, Write};
use std::net::Shutdown;
use std::str::from_utf8;
use std::{thread, time};

use i2p::sam::{SamConnection, SessionStyle, DEFAULT_API};

// Run with RUST_LOG=debug to see the action
#[tokio::main]
async fn main() {
	env_logger::init();
	let (pubkey, seckey) = {
		let mut sam_conn = SamConnection::connect(DEFAULT_API).unwrap();
		sam_conn
			.generate_destination(SignatureType::EdDsaSha512Ed25519)
			.unwrap()
	};
	info!("New public key: {}", pubkey);
	info!("New secret key: {}", seckey);
	let mut watcher = i2p::session_watcher::SamSessionWatcher::new(
		DEFAULT_API,
		&seckey,
		SessionStyle::Stream,
		Default::default(),
	)
	.unwrap();

	loop {
		match watcher.accept() {
			Ok((conn, addr)) => {
				info!("receiving incoming connection {}", addr);
				let _ = conn.shutdown(Shutdown::Both).unwrap();
			}
			Err(err) => {
				error!("failed to accept connection {:#?}", err);
			}
		}
	}
}
