# Number Converter

Simple base conversion project in Rust.

## Dependancies
Uses the `structopt` crate to handle command line input.

## Installation
Clone the repo and run `cargo build --release`.  The executable will be in the `target` directory.  Run from there, or move to a location in your `PATH` (`~/bin/` for example).

I alias the name to `ncon` and derivitaves, based on what base I'm inputing:
```
alias ncon="numconverter"
alias bcon="numconverter -b 2"
alias ocon="numconverter -b 8"
alias hcon="numconverter -b 16"
```

## Use
TODO: Document use.

For now `numconverter --help` will show you the options.

## TODO
- [x] ~~Add spacer option: insert a `_` every N characters (default 4)~~
- [x] ~~Add return codes~~
- [x] ~~Optional first argument for base~~
	- ~~Given as a single char (b, o, d, (h, x))~~
