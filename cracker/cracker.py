#!/usr/bin/env python3
"""
Native cracker — reimplements whiten()/xorshift64star() directly,
no subprocess calls needed.
"""

MASK64 = (1 << 64) - 1
GOLDEN = 0x9E3779B97F4A7C15
DEFAULT_NONCE = 0xA5A5_5A5A_D3C1_BEEF  # not used here; using known nonce from CLI default instead
NONCE = 11936045733561614063  # matches --nonce default used for target encoding

NYBBLE_TO_MEOW = [
    ":meow:", ":meow-love:", ":meow-yikes:", ":meow-sob:",
    ":meow-bread:", ":meow-sad:", ":meow-cry:", ":meow-melt:",
    ":meow-nerd:", ":meow-smart:", ":meow-ok:", ":meow-sweat:",
    ":meow-nwn:", ":meow-shocked:", ":meow-pat:", ":meow-coffee:",
]
TOKEN_TO_NIBBLE = {tok: i for i, tok in enumerate(NYBBLE_TO_MEOW)}

TARGET_TOKENS = """:meow-coffee: :meow-pat: :meow-melt: :meow-yikes: :meow-shocked: :meow-cry: :meow-nwn: :meow-nwn: :meow-melt: :meow-nwn: :meow-yikes: :meow-nerd: :meow-bread: :meow-sad: :meow-sob: :meow-pat: :meow-nerd: :meow-sob: :meow: :meow-coffee: :meow: :meow-ok: :meow-sweat: :meow-nerd: :meow-coffee: :meow-pat: :meow-nwn: :meow-nwn: :meow-cry: :meow-love: :meow-ok: :meow-sweat: :meow-bread: :meow-coffee: :meow-love: :meow-coffee: :meow-nwn: :meow-cry: :meow-smart: :meow: :meow-ok: :meow-bread: :meow-nerd: :meow-sweat: :meow-bread: :meow-smart:""".split()

LENGTH = len(TARGET_TOKENS) // 2

# Reconstruct ciphertext bytes from token pairs (order: high-nibble token, low-nibble token)
def tokens_to_bytes(tokens):
    out = bytearray()
    for i in range(0, len(tokens), 2):
        hi = TOKEN_TO_NIBBLE[tokens[i]]
        lo = TOKEN_TO_NIBBLE[tokens[i + 1]]
        out.append((hi << 4) | lo)
    return bytes(out)

CIPHERTEXT = tokens_to_bytes(TARGET_TOKENS)

def xorshift64star_stream(state, n):
    """Generate n keystream bytes, matching xorshift64star()."""
    s = state
    out = bytearray(n)
    for i in range(n):
        x = s
        x ^= (x >> 12)
        x &= MASK64
        x ^= (x << 25) & MASK64
        x &= MASK64
        x ^= (x >> 27)
        x &= MASK64
        s = x
        y = (x * 0x2545F4914F6CDD1D) & MASK64
        out[i] = y & 0xFF
    return bytes(out)

def whiten_keystream(key, nonce, n):
    s = (key ^ nonce ^ GOLDEN) & MASK64
    return xorshift64star_stream(s, n)

def try_key(key):
    ks = whiten_keystream(key, NONCE, LENGTH)
    pt = bytes(c ^ k for c, k in zip(CIPHERTEXT, ks))
    return pt

def is_printable_ascii(b):
    return all(0x20 <= c < 0x7F for c in b)

def main():
    found = []
    for key in range(100_000):
        pt = try_key(key)
        if is_printable_ascii(pt):
            found.append((key, pt))
        if key % 5000 == 0:
            print(f"\rTrying key {key}/100000", end="", flush=True)
    print()

    if not found:
        print("No candidates found.")
    else:
        for key, pt in found:
            print(f"key={key:05d}  plaintext={pt.decode()!r}")

if __name__ == "__main__":
    main()
