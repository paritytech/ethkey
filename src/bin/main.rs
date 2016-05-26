extern crate docopt;
extern crate rustc_serialize;
extern crate ethkey;

use std::{env, fmt};
use std::num::ParseIntError;
use docopt::Docopt;
use rustc_serialize::hex::{FromHex, FromHexError};
use ethkey::{KeyPair, Random, Brain, Prefix, Error as EthkeyError, Generator};

pub const USAGE: &'static str = r#"
Ethereum ABI coder.
  Copyright 2016 Ethcore (UK) Limited

Usage:
    ethkey generate random
    ethkey generate prefix <prefix> <iterations>
    ethkey generate brain <seed>
    ethkey [-h | --help]

Options:
    -h, --help         Display this message and exit.

Commands:
    generate           Generates new ethereum key.
    random             Random generation.
    prefix             Random generation, but address must start with a prefix
    brain              Generate new key from string seed.
"#;

#[derive(Debug, RustcDecodable)]
struct Args {
	cmd_generate: bool,
	cmd_random: bool,
	cmd_prefix: bool,
	cmd_brain: bool,
	arg_prefix: String,
	arg_iterations: String,
	arg_seed: String,
}

#[derive(Debug)]
enum Error {
	Ethkey(EthkeyError),
	FromHex(FromHexError),
	ParseInt(ParseIntError),
}

impl From<EthkeyError> for Error {
	fn from(err: EthkeyError) -> Self {
		Error::Ethkey(err)
	}
}

impl From<FromHexError> for Error {
	fn from(err: FromHexError) -> Self {
		Error::FromHex(err)
	}
}

impl From<ParseIntError> for Error {
	fn from(err: ParseIntError) -> Self {
		Error::ParseInt(err)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match *self {
			Error::Ethkey(ref e) => write!(f, "{}", e),
			Error::FromHex(ref e) => write!(f, "{}", e),
			Error::ParseInt(ref e) => write!(f, "{}", e),
		}
	}
}

fn main() {
	match execute(env::args()) {
		Ok(ok) => println!("{}", ok),
		Err(err) => println!("{}", err),
	}
}

fn execute<S, I>(command: I) -> Result<KeyPair, Error> where I: IntoIterator<Item=S>, S: AsRef<str> {
	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.argv(command).decode())
		.unwrap_or_else(|e| e.exit());

	return if args.cmd_generate {
		 if args.cmd_random {
			Random.generate().map_err(From::from)
		} else if args.cmd_prefix {
			let prefix = try!(args.arg_prefix.from_hex());
			let iterations = try!(usize::from_str_radix(&args.arg_iterations, 10));
			Prefix::new(prefix, iterations).generate().map_err(From::from)
		} else if args.cmd_brain {
			Brain::new(args.arg_seed).generate().map_err(From::from)
		} else {
			unreachable!();
		}
	} else {
		unreachable!();
	}
}


