#[macro_use]
extern crate nom;

pub use i2p::Socket;

mod i2p;
mod parsers;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use i2p::Socket;
        use i2p::SessionStyle;
        let socket = Socket::connect("127.0.0.1:7656");
        match socket {
            Ok(_) => println!("works"),
            Err(ref e) => println!("An error occurred: {}", e),
        }
        let mut foo = socket.unwrap();
        {
            let bar = foo.naming_lookup("zzz.i2p");
            match bar {
                Ok(_) => println!("works"),
                Err(ref e) => println!("An error occurred: {}", e),
            }
        }
        println!("It works!");
    }
}
