# Number Converter

Simple base conversion project in Rust.

This is a for-fun project designed to give me a taste of Rust, CI, and publishing/maintaining a repo.

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
- [ ] Filter out separation character from input string
- [ ] Set up CI
- [ ] Publish to crates.io

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
