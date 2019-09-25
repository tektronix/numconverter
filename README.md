# Number Converter

Simple base conversion project in Rust.

## Installation
Clone the repo and run `cargo build --release`.  The executable will be in the `target` directory.  Run from there, or move to a location in your `PATH` (`~/bin/` for example).

I alias the name to `ncon` and derivatives, based on what base I'm inputing:
```
alias ncon="numconverter"
alias bcon="numconverter -b 2"
alias ocon="numconverter -b 8"
alias hcon="numconverter -b 16"
```

## Dependancies
No dependanceies are required to build, other than [Rust](https://www.rust-lang.org/tools/install) itself.

Uses the `structopt` crate to handle command line input.

## Use
TODO: Document use.

For now `numconverter --help` will show you the options.

## TODO
- [x] ~~Add spacer option: insert specified character (default `_`) every N characters (default 4)~~
- [x] ~~Add return codes~~
- [x] ~~Optional first argument for base~~
	- ~~Given as a single char (b, o, d, (h, x))~~
