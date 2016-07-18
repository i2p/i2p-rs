#[macro_use]
extern crate nom;

pub use i2p::I2p;

mod i2p;
mod parsers;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use i2p::I2p;
        match I2p::connect("127.0.0.1:7656") {
            Ok(_) => println!("works"),
            Err(err) => println!("An error occurred: {}", err),
        }
    }
}
