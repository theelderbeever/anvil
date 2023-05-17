use clap::Parser;
use sha3::Digest;
use std::io::{self, Read};

fn print_version() -> &'static str {
    Box::leak(format!("v{}", env!("CARGO_PKG_VERSION")).into())
}

#[derive(Debug, Parser)]
#[command(name = "sha3")]
#[command(version = print_version(), about = "Sha3 hashing cli.", long_about = None)]
struct Sha3 {
    #[arg()]
    method: Sha3Methods,
    #[arg(help = "The string value to hash. Can also be piped to stdin.")]
    value: Option<String>,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Sha3Methods {
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Keccak224,
    Keccak256,
    Keccak384,
    Keccak512,
}

fn main() {
    let args = Sha3::parse();
    let value = match args.value {
        Some(v) => v,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            buffer
        }
    };

    let bytes = value.as_bytes();

    match args.method {
        Sha3Methods::Sha224 => {
            println!("{:x}", sha3::Sha3_224::digest(bytes))
        }
        Sha3Methods::Sha256 => {
            println!("{:x}", sha3::Sha3_256::digest(bytes))
        }
        Sha3Methods::Sha384 => {
            println!("{:x}", sha3::Sha3_384::digest(bytes))
        }
        Sha3Methods::Sha512 => {
            println!("{:x}", sha3::Sha3_512::digest(bytes))
        }
        Sha3Methods::Keccak224 => {
            println!("{:x}", sha3::Keccak224::digest(bytes))
        }
        Sha3Methods::Keccak256 => {
            println!("{:x}", sha3::Keccak256::digest(bytes))
        }
        Sha3Methods::Keccak384 => {
            println!("{:x}", sha3::Keccak384::digest(bytes))
        }
        Sha3Methods::Keccak512 => {
            println!("{:x}", sha3::Keccak512::digest(bytes))
        }
    };
}
