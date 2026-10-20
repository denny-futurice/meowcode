use std::{env, path::PathBuf, sync::LazyLock};

pub const DEFAULT_NONCE: u64 = 0xA5A5_5A5A_D3C1_BEEF;

pub static ASSETS_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    if let Some(dir) = env::var_os("MEOWCODE_ASSETS") {
        return PathBuf::from(dir);
    }
    let data_home = env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .unwrap_or_else(|| PathBuf::from("."));
    data_home.join("meowcode/assets")
});

pub const NYBBLE_TO_MEOW: [&str; 16] = [
    ":meow:",
    ":meow-love:",
    ":meow-yikes:",
    ":meow-sob:",
    ":meow-bread:",
    ":meow-sad:",
    ":meow-cry:",
    ":meow-melt:",
    ":meow-nerd:",
    ":meow-smart:",
    ":meow-ok:",
    ":meow-sweat:",
    ":meow-nwn:",
    ":meow-shocked:",
    ":meow-pat:",
    ":meow-coffee:",
];
