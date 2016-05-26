extern crate rand;
#[macro_use]
extern crate lazy_static;
extern crate tiny_keccak;
extern crate secp256k1;
extern crate rustc_serialize;

mod brain;
mod error;
mod keypair;
mod random;
mod prefix;

lazy_static! {
	static ref SECP256K1: secp256k1::Secp256k1 = secp256k1::Secp256k1::new();
}

pub type Address = [u8; 20];
pub type Secret  = [u8; 32];
pub type Public  = [u8; 64];

/// Generates new keypair.
pub trait Generator {
	/// Should be called to generate new keypair.
	fn generate(self) -> Result<KeyPair, Error>;
}

pub use self::brain::Brain;
pub use self::error::Error;
pub use self::keypair::KeyPair;
pub use self::prefix::Prefix;
pub use self::random::Random;
