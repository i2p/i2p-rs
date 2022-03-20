use env_logger;
use log::*;
use std::{thread, time};
use std::io::{Read, Write};
use std::str::from_utf8;

use i2p::net::{I2pListener, I2pStream};

fn main() {
	env_logger::init();

	// start a TCP server that will get forwards from i2p
	let server = I2pListener::bind().unwrap();
	let our_dest = server.local_addr().unwrap();
	thread::spawn(move || {
		for stream in server.incoming() {
			match stream {
				Ok(mut stream) => {
					thread::spawn(move || {
						let mut buffer = [0; 100];
						loop {
							let n = stream.read(&mut buffer).unwrap();
							info!("< {:?}", from_utf8(&buffer[0..n]).unwrap());
							stream.write("pong".as_bytes()).unwrap();
						}
					});
				}
				Err(e) => error!("Error on incoming connection: {:?}", e),
			}
		}
	});

	thread::sleep(time::Duration::from_millis(1000));

	// connect through i2p to our local destination
	let mut client = I2pStream::connect(our_dest).unwrap();
	let msg = "ping";
	client.write(msg.as_bytes()).unwrap();
	let mut buffer = [0; 100];
	let n = client.read(&mut buffer).unwrap();
	info!("> {:?}", from_utf8(&buffer[0..n]).unwrap());
}
