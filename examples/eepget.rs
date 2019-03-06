extern crate i2p;
extern crate env_logger;

use std::env;
use std::io::{BufReader, Read, Write};
use i2p::net::I2pStream;

fn help() {
    println!("Usage: eepget <host.i2p> [port]")
}

fn print_homepage(host: &str, port: u16) {
    let mut stream = I2pStream::connect(format!("{}:{}", host, port)).unwrap();

    let msg = "GET / HTTP/1.1\r\n\r\n";
    let _ = stream.write(msg.as_bytes());
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    let _ = reader.read_to_string(&mut buffer);

    println!("{}", buffer);
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => print_homepage(&args[1], 80),
        3 => {
            let host = &args[1];
            let port = &args[2];
            let port_num: u16 = match port.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Port must be an integer");
                    help();
                    return;
                }
            };
            print_homepage(host, port_num)
        }
        _ => help(),
    }
}
