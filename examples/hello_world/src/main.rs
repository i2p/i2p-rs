use crossbeam::sync::WaitGroup;
use env_logger;
use i2p;

use i2p::net::{I2pListener, I2pStream};
use i2p::sam_options::{SignatureType, SAMOptions, I2CPOptions, I2CPClientOptions, I2CPRouterOptions, I2CPTunnelInboundOptions, I2CPTunnelOutboundOptions};
use log::*;
use std::io::{Read, Write};
use std::net::Shutdown;
use std::str::from_utf8;
use std::{thread, time};
use crossbeam_channel::select;

use i2p::sam::{SamConnection, DEFAULT_API, SessionStyle};

// Run with RUST_LOG=debug to see the action
#[tokio::main]
async fn main() {
	env_logger::init();
	let (
		pubkey,
		seckey
	) = {
		let mut sam_conn = SamConnection::connect(DEFAULT_API).unwrap();
		sam_conn.generate_destination(SignatureType::EdDsaSha512Ed25519).unwrap()
	};
	info!("New public key: {}", pubkey);
	info!("New secret key: {}", seckey);
	let wg = WaitGroup::new();
	let (tx, rx) = crossbeam_channel::bounded::<bool>(1);
	{
		let sam_session = i2p::sam::Session::create(
			DEFAULT_API,
			seckey.as_str(),
			"hello_world",
			SessionStyle::Stream,
			SAMOptions {
				i2cp_options: Some(I2CPOptions {
					router_options: Some(I2CPRouterOptions {
						inbound: Some(I2CPTunnelInboundOptions {
							length: Some(1),
							quantity: Some(2),
							backup_quantity: Some(2),
							..Default::default()
						}),
						outbound: Some(I2CPTunnelOutboundOptions {
							length: Some(1),
							quantity: Some(2),
							backup_quantity: Some(2),
							..Default::default()
						}),
						..Default::default()
					}),
					..Default::default()
				}),
				signature_type: SignatureType::EdDsaSha512Ed25519,
				..Default::default()
			}
		).unwrap();
		let local_dest = i2p::net::I2pAddr::from_b64(&sam_session.local_dest).unwrap();
		info!("local_dest {}", local_dest);
		let rx = rx;
		let listener = match I2pListener::bind_with_session(&sam_session) {
			Ok(listener) => listener,
			Err(err) => panic!("failed to establish listener with session {:#?}", err),
		};
		let wg = wg.clone();
		tokio::task::spawn_blocking(move || {
			loop {
				select! {
					recv(rx) -> _msg => {
						warn!("server received exit signal, goodbye...");
						match sam_session.sam.conn.shutdown(Shutdown::Both) {
							Ok(_) => info!("server shutdown ok"),
							Err(err) => error!("server failed to properly shutdown {:#?}", err)
						}
						drop(wg);
						return;
					},
					default() => {}
				}
				match listener.accept() {
					Ok((mut incoming_conn, conn_addr)) => {
						info!("server accepted connection from {}", conn_addr);
						let mut buf = [0_u8; 512];
						match incoming_conn.read(&mut buf) {
							Ok(n) => {
								// dont do this outside of an example
								unsafe { info!("server read {} bytes. msg {}", n, String::from_utf8_unchecked(buf[0..n].to_vec()).replace("\n", "")); }
								match incoming_conn.write(&buf[0..n]) {
									Ok(n) => {
										info!("server wrote {} bytes", n)
									}
									Err(err) => {
										error!("server failed to write response for {}: {:#?}", conn_addr, err);	
									}
								}
							}
							Err(err) => {
								error!("server failed to read data from {}: {:#?}", conn_addr, err);
							}
						}
						
					},
					Err(err) => {
						error!("server failed to accept connection {:#?}", err);
					}
				}
				// because we called spawn_blocking this wont block main thread
				std::thread::sleep(std::time::Duration::from_millis(125));
			}
		});
	}
	info!("waiting 10 seconds for tunnel things to happen");
	// because we used tokio::task::spawn_blocking, we can sleep here 
	// or in the spawned task without either sleeping blocking a thread
	// and pausing execution.
	std::thread::sleep(std::time::Duration::from_secs(10));
	let mut client_conn = match I2pStream::connect(&format!("{}:0", pubkey)) {
		Ok(client_conn) => client_conn,
		Err(err) => {
			if let Err(err) = tx.send(true)  {
				error!("client failed to signal server task to exit {:#?}", err);
			}
			panic!(
				"client failed to connect to destination {}, {:#?}",
				i2p::net::I2pAddr::from_b64(&pubkey).unwrap(),
				err
			);
		}
	};
	match client_conn.write(b"hello_world") {
		Ok(n) => info!("client wrote {} bytes", n),
		Err(err) => {
			if let Err(err) = tx.send(true)  {
				error!("client failed to signal server task to exit {:#?}", err);
			}
			panic!("client failed to write into stream {:#?}", err);	
		}
	}
	let mut buf = [0_u8; 512];
	match client_conn.read(&mut buf) {
		Ok(n) => {
			if let Err(err) = tx.send(true)  {
				info!("client failed to signal server task to exit {:#?}", err);
			}
			unsafe { info!("client read {} bytes. msg {}", n, String::from_utf8_unchecked(buf[0..n].to_vec()).replace("\n", "")); }
		}
		Err(err) => {
			if let Err(err) = tx.send(true)  {
				error!("client failed to signal server task to exit {:#?}", err);
			}
			panic!("client failed to read from stream {:#?}", err);	
		}
	}
	wg.wait();
	match client_conn.shutdown(Shutdown::Both) {
		Ok(_) => info!("client shutdown ok"),
		Err(err) => error!("client failed to properly shutdown {:#?}", err)
	}
	info!("all background processes exited, goodbye...");
	
}
