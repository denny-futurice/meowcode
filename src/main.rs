mod bytewrangling;
mod consts;
mod meowcoder;

use clap::Parser;
use clap::Subcommand;
use std::io::{self, stdin, BufRead};

use crate::consts::DEFAULT_NONCE;
use crate::meowcoder::{decode_to_boring, encode_to_meows};

fn encode_from_stdio(kitty_enabled: bool, key: u64, nonce: u64) {
    let mut input: String = String::new();
    loop {
        match stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                if !input.is_empty() {
                    encode_to_meows(&input, kitty_enabled, key, nonce);
                }
                input.clear();
            }
            Err(error) => {
                eprintln!("ERR: {}", error)
            }
        }
    }
}

fn decode_from_stdio(key: u64, nonce: u64) -> io::Result<()> {
    let stdin = stdin();
    let mut input = String::new();
    let mut handle = stdin.lock();

    loop {
        input.clear();
        let n = handle.read_line(&mut input)?;
        if n == 0 {
            break;
        }

        if !input.trim().is_empty() {
            decode_to_boring(&input, key, nonce);
        }
    }

    Ok(())
}

// FIXME: Change this so that boring and meow are not needed
#[derive(Subcommand, Debug)]
enum Commands {
    Encode {
        #[arg(short, long)]
        boring: String,
        #[arg(short, long)]
        kitty_enabled: bool,
        #[arg(long, default_value_t = 0)]
        key: u64,
        #[arg(long, default_value_t = DEFAULT_NONCE)]
        nonce: u64,
    },
    Decode {
        #[arg(long)]
        meow: String,
        #[arg(long, default_value_t = 0)]
        key: u64,
        #[arg(long, default_value_t = DEFAULT_NONCE)]
        nonce: u64,
    },
    StdioEncoder {
        #[arg(short, long)]
        kitty_enabled: bool,
        #[arg(long, default_value_t = 0)]
        key: u64,
        #[arg(long, default_value_t = DEFAULT_NONCE)]
        nonce: u64,
    },
    StdioDecoder {
        #[arg(long, default_value_t = 0)]
        key: u64,
        #[arg(long, default_value_t = DEFAULT_NONCE)]
        nonce: u64,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Some(Commands::Decode { meow, key, nonce }) => decode_to_boring(meow, *key, *nonce),
        Some(Commands::Encode {
            boring,
            kitty_enabled,
            key,
            nonce,
        }) => encode_to_meows(boring, *kitty_enabled, *key, *nonce),
        Some(Commands::StdioEncoder {
            kitty_enabled,
            key,
            nonce,
        }) => encode_from_stdio(*kitty_enabled, *key, *nonce),
        Some(Commands::StdioDecoder { key, nonce }) => {
            decode_from_stdio(*key, *nonce).expect("FAILURE")
        }
        _ => eprintln!("Oh noes"),
    }
}
