#[test]
#[ignore]
fn naming_lookup() {
	use i2p::sam::SamConnection;
	let mut sam = SamConnection::connect("127.0.0.1:7656").unwrap();
	{
		let res = sam.naming_lookup("zzz.i2p");
		match res {
			Ok(d) => {
				assert_eq!(
                    d,
                    "GKapJ8koUcBj~jmQzHsTYxDg2tpfWj0xjQTzd8BhfC9c3OS5fwPBNajgF-eOD6eCjFTqTlorlh7Hnd8kXj1qblUGXT-tDoR9~YV8dmXl51cJn9MVTRrEqRWSJVXbUUz9t5Po6Xa247Vr0sJn27R4KoKP8QVj1GuH6dB3b6wTPbOamC3dkO18vkQkfZWUdRMDXk0d8AdjB0E0864nOT~J9Fpnd2pQE5uoFT6P0DqtQR2jsFvf9ME61aqLvKPPWpkgdn4z6Zkm-NJOcDz2Nv8Si7hli94E9SghMYRsdjU-knObKvxiagn84FIwcOpepxuG~kFXdD5NfsH0v6Uri3usE3XWD7Pw6P8qVYF39jUIq4OiNMwPnNYzy2N4mDMQdsdHO3LUVh~DEppOy9AAmEoHDjjJxt2BFBbGxfdpZCpENkwvmZeYUyNCCzASqTOOlNzdpne8cuesn3NDXIpNnqEE6Oe5Qm5YOJykrX~Vx~cFFT3QzDGkIjjxlFBsjUJyYkFjBQAEAAcAAA=="
                )
			}
			Err(ref e) => panic!("An error occurred: {e}"),
		}
	}
}
