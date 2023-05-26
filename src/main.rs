use clap::Parser;
use colored::Colorize;
use sha3::{
    Digest, Keccak224, Keccak256, Keccak384, Keccak512, Sha3_224, Sha3_256, Sha3_384, Sha3_512,
};
use std::io::{self};

fn print_version() -> &'static str {
    Box::leak(format!("v{}", env!("CARGO_PKG_VERSION")).into())
}

#[derive(Debug, Parser)]
#[command(name = "sha3")]
#[command(version = print_version(), about = "Sha3 hashing cli.\n\n❯ echo -n 'some text' | sha3cli sha256\n802a5a961895b3f8c6556e31d0960a5778d7135be7d04bbbadd5e406c4bac381\n\n❯ sha3 sha256 'some text'\nn802a5a961895b3f8c6556e31d0960a5778d7135be7d04bbbadd5e406c4bac381", arg_required_else_help = true,)]
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
            if atty::is(atty::Stream::Stdin) {
                let err = "ERROR".bold().red();
                eprintln!("{err}: Missing argument <VALUE>");
                return;
            }
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            buffer
        }
    };

    let bytes = value.as_bytes();

    let hash = match args.method {
        Sha3Methods::Sha224 => format!("{:x}", Sha3_224::digest(bytes)),
        Sha3Methods::Sha256 => format!("{:x}", Sha3_256::digest(bytes)),
        Sha3Methods::Sha384 => format!("{:x}", Sha3_384::digest(bytes)),
        Sha3Methods::Sha512 => format!("{:x}", Sha3_512::digest(bytes)),
        Sha3Methods::Keccak224 => format!("{:x}", Keccak224::digest(bytes)),
        Sha3Methods::Keccak256 => format!("{:x}", Keccak256::digest(bytes)),
        Sha3Methods::Keccak384 => format!("{:x}", Keccak384::digest(bytes)),
        Sha3Methods::Keccak512 => format!("{:x}", Keccak512::digest(bytes)),
    };

    println!("{hash}");
}
