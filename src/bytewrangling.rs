pub fn split_nybbles(char: &u8) -> (u8, u8) {
    (char >> 4, char & 0x0F)
}

pub fn combine_nybbles_to_byte(high: u8, low: u8) -> u8 {
    (high << 4) | low
}

pub fn xorshift64star(state: &mut u64) -> u8 {
    let mut x = *state;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    *state = x;
    let y = x.wrapping_mul(0x2545F4914F6CDD1D);
    (y & 0xFF) as u8
}

pub fn whiten(data: &[u8], key: u64, nonce: u64) -> Vec<u8> {
    let mut s = key ^ nonce ^ 0x9E3779B97F4A7C15;
    data.iter().map(|b| b ^ xorshift64star(&mut s)).collect()
}
