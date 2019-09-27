[![TekMonogram](https://tektronix.github.io/media/tekmonogram.png)](https://github.com/tektronix)

# Number Converter
[![Tektronix](https://tektronix.github.io/media/TEK-opensource_badge.svg)](https://github.com/tektronix)  [![CodeFactor](https://www.codefactor.io/repository/github/tektronix/numconverter/badge)](https://www.codefactor.io/repository/github/tektronix/numconverter) 

Simple base conversion project in Rust.

This is a for-fun project designed to give me a taste of Rust, CI, and publishing/maintaining a repo.

## Installation
Clone the repo and run `cargo build --release`.  The executable `numconverter` will be in the `target/release` directory.  Run from there, or move/symlink to a location in your `PATH` (`~/bin/` for example).

I alias the name to `ncon` and derivatives, based on what base I'm inputing:
```
alias ncon="numconverter"
alias bcon="numconverter --from-base 2"
alias ocon="numconverter --from-base 8"
alias hcon="numconverter --from-base 16"
```

## Dependancies
Install Rust and you will have everything you need ([Rust install instructions](https://www.rust-lang.org/tools/install)).

Uses the `structopt` crate to handle command line input.

## Use
TODO: Document use.

For now `numconverter --help` will show you the options.

```
numconverter [base_char] <from_num> [to_base]... [FLAGS]... [OPTIONS]...

ARGS:
    base_char   (b, o, d, h - binary, octal, decimal, hex) Character
                representing the 'from' base.  If both the base_char and
                the -f/--from-base are provided, base_char will be used.
    from_num    The input number to convert.  Default base 10.
    to_base     A list of base 10 numbers to convert from_num to.  Base
                must be between 2 and 32 inclusive.

FLAGS:
        --bare          Disable Pretty Print
    -h, --help          Prints help information
        --no-sep        Do not pad the output
    -s, --silent        Do not print output, for use with the clipboard on.
                        Clipboard functionality not supported currently.
    -V, --version       Prints version information
    -v, --verbosity     Verbosity (more v's, more verbose)


OPTIONS:
    -f, --from-base <from-base>      Base of the input number [default: 10]
    -p, --pad <pad>                  Pad the output with leading 0s [default: 0]
        --sep-char <sep-char>        Specify spacer char [default: _]
    -l, --sep-length <sep-length>    Put a spacer every N characters [default: 4]
```


## Contributing
Before submitting a PR, please make sure you have run
* `cargo test` - Runs defined tests.  Be sure to add test coverage for your PR! 😁
* Ensure there are no warnings when compiling.

### License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

#### Contributor License Agreement
Contributions to this project must be accompanied by a Contributor License Agreement. You (or your employer) retain the copyright to your contribution; this simply gives us permission to use and redistribute your contributions as part of the project.

### Maintainers
[Zach Nielsen](https://github.com/ZatchBo) - @ZatchBo
