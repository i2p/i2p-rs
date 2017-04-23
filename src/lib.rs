#[macro_use]
extern crate nom;

pub use sam::Socket;

mod sam;
mod parsers;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use sam::Socket;
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
