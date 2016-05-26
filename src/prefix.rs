use super::{Random, Generator, KeyPair, Error};

/// Tries to find keypair with address starting with given prefix.
pub struct Prefix(Vec<u8>);

impl Generator for Prefix {
	fn generate(self) -> Result<KeyPair, Error> {
		loop {
			let keypair = try!(Random.generate());
			if keypair.address().starts_with(&self.0) {
				return Ok(keypair)
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use {Generator, Prefix};

	#[test]
	fn prefix_generator() {
		let prefix = vec![0xffu8];
		let keypair = Prefix(prefix.clone()).generate().unwrap();
		assert!(keypair.address().starts_with(&prefix));
	}
}
