# rreader

Compressing text with RePair and outputting them in a different format.

This uses Gonzalo Navarro's implementation of the RePair algorithm which can be found [here](https://users.dcc.uchile.cl/~gnavarro/software/).

## Build instructions

1. Install [rust](https://www.rust-lang.org/).
2. Compile the project using `cargo build` (Debug) or `cargo build --release` (Release)
3. The binary will be in `target/debug` or `target/release` respectively

## Usage

If a file `example.txt` should be compressed,
either run the project with `cargo run -- example.txt` or run the binary `./rreader example.txt`.
A file called `example.txt.grm` will be created containing the (somewhat) compressed text.

Decompression is WIP.

## File format

The compressed file format is as follows:
The first value is a 32-bit unsigned integer which is the number of rules.

Then for each rule:
1. 32-bit unsigned integer representing the number of symbols.
2. For each symbol:
  - 0 bit if it is a terminal (<256), or 1 bit if it is a non-terminal
  - if it's a terminal an 8-bit extended ascii symbol follows
  - if it's a non-terminal a 32-bit number follows, representing the id of the rule offset by 256

