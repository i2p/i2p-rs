pub fn rand_string(length: usize) -> String {
	use rand::distributions::{Alphanumeric, DistString};
	Alphanumeric.sample_string(&mut rand::thread_rng(), length)
}
