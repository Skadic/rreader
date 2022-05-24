# rreader

Compressing text with RePair and outputting them in a different format.

This uses Gonzalo Navarro's implementation of the RePair algorithm which can be found [here](https://users.dcc.uchile.cl/~gnavarro/software/).

## Build instructions

1. Install [rust](https://www.rust-lang.org/).
2. Compile the project using `cargo build` (Debug) or `cargo build --release` (Release)
3. The binary will be in `target/debug` or `target/release` respectively

## Usage

Help can be displayed by running the project with the `--help` flag.

```
rreader 0.1.0

USAGE:
    rreader [OPTIONS] --file <FILE>

OPTIONS:
    -d, --decompress     Decompress the input file
    -f, --file <FILE>    The input file
    -h, --help           Print help information
    -o, --out <OUT>      The output file
    -V, --version        Print version information
```

### Compression

If a file `example.txt` should be compressed,
either run the project with `cargo run -- -f example.txt` or run the binary `./rreader -f example.txt`.
A file called `example.txt.grm` (default output name just appends `.grm`) will be created containing the (somewhat) compressed text.

### Decompression

Decompression can be done by using the `-d` flag.
Note that specifying the output file name (`-o`) is required when decompressing.
Either run the project with `cargo run -- -d -f compressed_example.txt -o decompressed.txt` or run the binary `./rreader -d -f compressed_example.txt -o decompressed.txt`.

## File format

The compressed file format is as follows:
The first value is a 32-bit unsigned integer which is the number of rules.

Then for each rule:
1. 32-bit unsigned integer representing the number of symbols.
2. For each symbol:
  - 0 bit if it is a terminal (<256), or 1 bit if it is a non-terminal
  - if it's a terminal an 8-bit extended ascii symbol follows
  - if it's a non-terminal a 32-bit number follows, representing the id of the rule offset by 256

