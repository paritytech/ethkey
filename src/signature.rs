use std::ops::{Deref, DerefMut};
use std::mem;
use secp256k1::Message;
use secp256k1::key::SecretKey;
use {Secret, SECP256K1, Error};

#[repr(C)]
pub struct Signature {
	pub r: [u8; 32],
	pub s: [u8; 32],
	pub v: u8,
}

impl Default for Signature {
	fn default() -> Self {
		Signature {
			r: [0u8; 32],
			s: [0u8; 32],
			v: 0u8,
		}
	}
}

impl From<[u8; 65]> for Signature {
	fn from(s: [u8; 65]) -> Self {
		unsafe { mem::transmute(s) }
	}
}

impl Into<[u8; 65]> for Signature {
	fn into(self) -> [u8; 65] {
		unsafe { mem::transmute(self) }
	}
}

impl Deref for Signature {
	type Target = [u8; 65];

	fn deref(&self) -> &Self::Target {
		unsafe { mem::transmute(self) }
	}
}

impl DerefMut for Signature {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { mem::transmute(self) }
	}
}

pub fn sign(secret: &Secret, message: &[u8; 32]) -> Result<Signature, Error> {
	let context = &SECP256K1;
	// no way to create from raw byte array.
	let sec: &SecretKey = unsafe { mem::transmute(secret) };
	let s = try!(context.sign_recoverable(&try!(Message::from_slice(message)), sec));
	let (rec_id, data) = s.serialize_compact(context);
	let mut signature = Signature::default();
	signature.r.copy_from_slice(&data[0..32]);
	// no need to check if s is low, it alawys is
	signature.s.copy_from_slice(&data[32..64]);
	signature.v = rec_id.to_i32() as u8;
	Ok(signature)
}
