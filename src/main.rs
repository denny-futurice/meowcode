use clap::Parser;
use clap::Subcommand;
use kitty_image::{Action, ActionTransmission, Command, Format, Medium, WrappedCommand};
use regex::Regex;
use std::io;

fn split_nybbles(char: &u8) -> (u8, u8) {
    return (char >> 4, char & 0x0F);
}

// FIXME: Figure out something so that this is a bidirectional hash.

fn map_nybble_to_meow(byte: u8) -> String {
    let result = match byte {
        0u8 => ":meow:",
        1u8 => ":meow-love:",
        2u8 => ":meow-yikes:",
        3u8 => ":meow-sob:",
        4u8 => ":meow-bread:",
        5u8 => ":meow-sad:",
        6u8 => ":meow-cry:",
        7u8 => ":meow-melt:",
        8u8 => ":meow-nerd:",
        9u8 => ":meow-smart:",
        10u8 => ":meow-ok:",
        11u8 => ":meow-sweat:",
        12u8 => ":meow-nwn:",
        13u8 => ":meow-shocked:",
        14u8 => ":meow-pat:",
        15u8 => ":meow-coffee:",
        _ => "invalid-meow:",
    };
    result.to_string()
}

fn map_meow_to_nybble(meow: &str) -> u8 {
    let result = match meow {
        ":meow:" => 0u8,
        ":meow-love:" => 1u8,
        ":meow-yikes:" => 2u8,
        ":meow-sob:" => 3u8,
        ":meow-bread:" => 4u8,
        ":meow-sad:" => 5u8,
        ":meow-cry:" => 6u8,
        ":meow-melt:" => 7u8,
        ":meow-nerd:" => 8u8,
        ":meow-smart:" => 9u8,
        ":meow-ok:" => 10u8,
        ":meow-sweat:" => 11u8,
        ":meow-nwn:" => 12u8,
        ":meow-shocked:" => 13u8,
        ":meow-pat:" => 14u8,
        ":meow-coffee:" => 15u8,
        _ => 255u8,
    };
    return result;
}

fn combine_nybbles_to_byte(high: u8, low: u8) -> u8 {
    (high << 4) | low
}

fn character_to_meow_meow(character: &u8, kitty: &bool) -> String {
    let (low, high) = split_nybbles(character);
    if *kitty == true {
        return format!(
            " {}  {} ",
            emit_kitty_emoji(&map_nybble_to_meow(low)),
            emit_kitty_emoji(&map_nybble_to_meow(high))
        );
    } else {
        return format!("{} {}", map_nybble_to_meow(low), map_nybble_to_meow(high));
    }
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

fn encode_to_meows(boring: &String, kitty: &bool) {
    let boring_bytes = boring.as_bytes();
    for byte in boring_bytes.iter() {
        print!("{} ", character_to_meow_meow(byte, kitty))
    }
}

fn emit_kitty_emoji(emoji: &String) -> String {
    let emoji_file_path = format!("assets/{}.png", emoji.replace(":", ""));
    let wrapped_command = WrappedCommand::new(Command::with_payload_from_path(
        Action::TransmitAndDisplay(
            ActionTransmission {
                format: Format::Png,
                medium: Medium::File,
                height: 16,
                width: 16,
                ..Default::default()
            },
            kitty_image::ActionPut::default(),
        ),
        emoji_file_path.as_ref(),
    ));

    let wrapped_string = wrapped_command.to_string();
    //println!("DEBUG: {} -> {}", emoji, wrapped_string);
    return wrapped_string;
}

fn encode_from_stdio(kitty_enabled: &bool) {
    let mut input: String = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                if input.len() > 0 {
                    encode_to_meows(&input, kitty_enabled);
                }
                input.clear();
            }
            Err(error) => {
                println!("ERR: {}", error)
            }
        }
    }
}

fn decode_from_stdio() {
    let mut input: String = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                if input.len() > 0 {
                    decode_to_boring(&input);
                }
                input.clear();
            }
            Err(error) => {
                println!("ERR: {}", error)
            }
        }
    }
}

// FIXME: Change this so that boring and meow are not needed
#[derive(Subcommand, Debug)]
enum Commands {
    Encode {
        #[arg(short, long)]
        boring: String,
        #[arg(short, long)]
        kitty_enabled: bool,
    },
    Decode {
        #[arg(long)]
        meow: String,
    },
    StdioEncoder {
        #[arg(short, long)]
        kitty_enabled: bool,
    },
    StdioDecoder {},
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
        Some(Commands::Encode {
            boring,
            kitty_enabled,
        }) => encode_to_meows(boring, &kitty_enabled.clone()),
        Some(Commands::StdioEncoder { kitty_enabled }) => encode_from_stdio(&kitty_enabled),
        Some(Commands::StdioDecoder { .. }) => decode_from_stdio(),
        _ => println!("Oh noes"),
    }
}
