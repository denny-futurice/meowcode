# Meowcode


![meow](screenshot.png "Meow")

Convert boring text to streams of :meow: for slack messages and such meowfoolery.

As meme demands, blazingly fast.

Simple meowcryption included so meow sequences look random and a simple key can be provided.

to encode text:

```shell
meowcode --key 12345 --encode something
meowcode --key 12345 --decode ":meow-nwn: :meow-pat: :meow-melt: :meow-nwn: :meow-nwn: :meow-coffee:"
```

You can also pipe in stdin and print out emoji's line by line:

```sh
cat /dev/urandom | meowcode stdio-encoder --key 12345 
```

Kitty rendering of emoji's is supported with `--kitty-enabled` but is only practical for short sequences.


## Building

Rust stable channel, cargo build, :meow-smug:.

Tests? Cat's do not test.

## Installing assets

To make use of the kitty emoji output you need to install the assets.

These get dumped to `${XDG_DATA_HOME}/meowcode/assets` (which is typically in `~/.local/share/`)

To do so, run the included makefile `make install`.

## Cracker

Just so you don't trust this, because.. who would trust a cat?!

A cracker and the claude session that generated it are included in `cracker/`

## AI usage

Meow's rolled by hand, the crack was done with Claude as an experiment to evaluate how long it would take.

## License

WTFPL what else
