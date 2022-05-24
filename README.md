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
