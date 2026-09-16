use clap::Parser;
use clap::Subcommand;
use regex::Regex;

fn split_nybbles(char: &u8) -> (u8, u8) {
    return (char >> 4, char & 0x0F);
}

// FIXME: Figure out something so that this is a bidirectional hash.

fn map_nybble_to_meow(byte: u8) -> String {
    let result = match byte {
        0u8 => ":meow:",
        1u8 => ":meow_heart:",
        2u8 => ":meow_dead:",
        3u8 => ":meow_code:",
        4u8 => ":meow-smug:",
        5u8 => ":meow-sad:",
        6u8 => ":meow-sleepy:",
        7u8 => ":meow-beer:",
        8u8 => ":meow-give:",
        9u8 => ":meow-smart:",
        10u8 => ":meow-ok:",
        11u8 => ":meow-pizza:",
        12u8 => ":meow-shy:",
        13u8 => ":meow-shocked:",
        14u8 => ":meow-nod:",
        15u8 => ":meow-coffee:",
        _ => "invalid-meow:",
    };
    result.to_string()
}

fn map_meow_to_nybble(meow: &str) -> u8 {
    let result = match meow {
        ":meow:" => 0u8,
        ":meow_heart:" => 1u8,
        ":meow_dead:" => 2u8,
        ":meow_code:" => 3u8,
        ":meow-smug:" => 4u8,
        ":meow-sad:" => 5u8,
        ":meow-sleepy:" => 6u8,
        ":meow-beer:" => 7u8,
        ":meow-give:" => 8u8,
        ":meow-smart:" => 9u8,
        ":meow-ok:" => 10u8,
        ":meow-pizza:" => 11u8,
        ":meow-shy:" => 12u8,
        ":meow-shocked:" => 13u8,
        ":meow-nod:" => 14u8,
        ":meow-coffee:" => 15u8,
        _ => 255u8,
    };
    return result;
}

fn combine_nybbles_to_byte(high: u8, low: u8) -> u8 {
    (high << 4) | low
}

fn character_to_meow_meow(character: &u8) -> String {
    let (low, high) = split_nybbles(character);
    format!("{} {}", map_nybble_to_meow(low), map_nybble_to_meow(high))
}

fn decode_to_boring(meow: &String) {
    // FIXME: Instead of take a &String as an argument,
    // take a stdio.reader instead and stream bytes.

    let regex = Regex::new(":*:").unwrap();
    let mut pair: Vec<u8> = Vec::new();
    for token in regex.split(meow) {
        if pair.len() == 2 {
            let low = pair.pop().unwrap();
            let high = pair.pop().unwrap();
            let decoded = combine_nybbles_to_byte(high, low) as char;
            print!("{}", decoded)
        }
        if token != " " && token != "" {
            let emoji = format!(":{}:", token);
            let decoded = map_meow_to_nybble(&emoji);
            pair.push(decoded)
        }
    }
}

fn encode_to_meows(boring: &String) {
    let boring_bytes = boring.as_bytes();
    for byte in boring_bytes.iter() {
        print!("{} ", character_to_meow_meow(byte));
    }
}

// FIXME: Change this so that boring and meow are not needed
#[derive(Subcommand, Debug)]
enum Commands {
    Encode {
        #[arg(short, long)]
        boring: String,
    },
    Decode {
        #[arg(long)]
        meow: String,
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
        Some(Commands::Decode { meow }) => decode_to_boring(meow),
        Some(Commands::Encode { boring }) => encode_to_meows(boring),
        _ => println!("Oh noes"),
    }
}
