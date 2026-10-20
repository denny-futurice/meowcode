use crate::bytewrangling::{combine_nybbles_to_byte, split_nybbles, whiten};
use crate::consts::NYBBLE_TO_MEOW;
use kitty_image::{Action, ActionTransmission, Command, Format, Medium, WrappedCommand};
use regex::Regex;

pub fn map_nybble_to_meow(n: u8) -> String {
    NYBBLE_TO_MEOW
        .get(n as usize)
        .copied()
        .unwrap_or("invalid-meow:")
        .to_string()
}

pub fn map_meow_to_nybble(meow: &str) -> u8 {
    NYBBLE_TO_MEOW
        .iter()
        .position(|&m| m == meow)
        .map(|i| i as u8)
        .unwrap_or(255)
}

pub fn character_to_meow_meow(character: &u8, kitty: &bool) -> String {
    let (high, low) = split_nybbles(character);
    if *kitty {
        format!(
            " {}  {} ",
            emit_kitty_emoji(&map_nybble_to_meow(high)),
            emit_kitty_emoji(&map_nybble_to_meow(low))
        )
    } else {
        format!("{} {}", map_nybble_to_meow(high), map_nybble_to_meow(low))
    }
}

pub fn emit_kitty_emoji(emoji: &str) -> String {
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
    wrapped_command.to_string()
}

pub fn decode_to_boring(meow: &str, key: u64, nonce: u64) {
    let regex = Regex::new(":*:").unwrap();
    let mut pair: Vec<u8> = Vec::new();
    let mut decoded_bytes: Vec<u8> = Vec::new();

    for token in regex.split(meow) {
        if pair.len() == 2 {
            let low = pair.pop().unwrap();
            let high = pair.pop().unwrap();
            let decoded = combine_nybbles_to_byte(high, low);
            decoded_bytes.push(decoded);
        }
        if token != " " && !token.is_empty() {
            let emoji = format!(":{}:", token);
            let decoded = map_meow_to_nybble(&emoji);
            pair.push(decoded);
        }
    }

    if pair.len() == 2 {
        let low = pair.pop().unwrap();
        let high = pair.pop().unwrap();
        decoded_bytes.push(combine_nybbles_to_byte(high, low));
    }

    let plain = whiten(&decoded_bytes, key, nonce);
    print!("{}", String::from_utf8_lossy(&plain));
}

pub fn encode_to_meows(boring: &str, kitty: bool, key: u64, nonce: u64) {
    let whitened = whiten(boring.as_bytes(), key, nonce);
    for byte in whitened.iter() {
        print!("{} ", character_to_meow_meow(byte, &kitty));
    }
}
