use tiny_keccak::Keccak;
use super::{KeyPair, Error, Generator};

/// Simple brainwallet.
pub struct Brain(String);

impl Brain {
	pub fn new(s: String) -> Self {
		Brain(s)
	}
}

impl Generator for Brain {
	fn generate(self) -> Result<KeyPair, Error> {
		let seed = self.0;
		let data: Vec<u8> = seed.bytes().collect();

		let mut keccak = Keccak::new_keccak256();
		keccak.update(&data);

		let mut secret = [0u8; 32];
		keccak.finalize(&mut secret);

		let mut i = 0;
		loop {
			let mut keccak = Keccak::new_keccak256();
			keccak.update(&secret);
			let mut new_secret = [0u8; 32];
			keccak.finalize(&mut new_secret);
			secret = new_secret;
			
			match i > 16384 {
				false => i += 1,
				true => {
					let result = KeyPair::from_secret(secret.clone());
					if result.is_ok() {
						return result
					}
				},
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use {Brain, Generator};

	#[test]
	fn test_brain() {
		let words = "this is sparta!".to_owned();
		let first_keypair = Brain(words.clone()).generate().unwrap();
		let second_keypair = Brain(words.clone()).generate().unwrap();
		assert_eq!(first_keypair.secret(), second_keypair.secret());
	}
}
