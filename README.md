[![TekMonogram](https://tektronix.github.io/media/tekmonogram.png)](https://github.com/tektronix)

# Number Converter
[![Tektronix](https://tektronix.github.io/media/TEK-opensource_badge.svg)](https://github.com/tektronix)  [![CodeFactor](https://www.codefactor.io/repository/github/tektronix/numconverter/badge)](https://www.codefactor.io/repository/github/tektronix/numconverter) [![Build Status](https://travis-ci.com/tektronix/numconverter.svg?branch=master)](https://travis-ci.com/tektronix/numconverter)

A simple base conversion project in Rust.

## Installation

### Crates.io
Numconverter is on Crates.io! Just install via
```
cargo install numconverter
```

### From source
Clone the repo and run `cargo build --release`.  The executable `numconverter` will be in the `target/release` directory.  Run from there, or move/symlink to a location in your `PATH` (`~/bin/` for example).

I alias the name to `ncon` and derivatives, based on what base I'm inputing:
```
alias ncon="numconverter"
alias bcon="numconverter --from-base 2"
alias ocon="numconverter --from-base 8"
alias hcon="numconverter --from-base 16"
```

## Dependencies
Built using Rust. ([Rust installation instructions](https://www.rust-lang.org/tools/install)).
- Uses the `structopt` crate to handle command line input.
- Uses the `clipboard` crate to handle read and write to clipboard. [For Linux: `x11_clipboard` & `nix` crates]

#### Linux
Dependencies for clipboard functionality include:
- libxcb-shape0-dev
- libxcb-xfixes0-dev

Install with
```
sudo apt-get install libxcb-shape0-dev libxcb-xfixes0-dev
```


## Usage

Some basic usage examples are provided below:
<details><summary><b>Show Examples</b></summary>

Basic decimal conversion
```
$ numconverter 255
Base 02: 1111_1111
Base 08: 377
Base 10: 255
Base 16: FF
```

Specify binary input
```
$ numconverter b 1001
Base 02: 1001
Base 08: 11
Base 10: 9
Base 16: 9
```

Specify hexidecimal input
```
$ numconverter h ab12
Base 02: 1010_1011_0001_0010
Base 08: 12_5422
Base 10: 4_3794
Base 16: AB12
```

Specify output base - binary (base 2) to hexidecimal (base 16)
```
$ numconverter b 1010 16
Base 16: A
```

Specify output bases (non standard)
```
$ numconverter 1234567890 3 5 12 22 32
Base 03: 1001_2001_0011_1220_2200
Base 05: 10_0120_2213_3030
Base 12: 2_A555_5016
Base 22: AJC_3E26
Base 32: 14P_C0MI
```

</details>

Enter `numconverter --help` for available options.

```
numconverter [base_char] [from_num] [to_base]... [FLAGS]... [OPTIONS]...

ARGS:
    base_char   (b, o, d, h - binary, octal, decimal, hex) Character
                representing the 'from' base.  If both the base_char and
                the -f/--from-base are provided, base_char will be used.
    from_num    The input number to convert.  Default base 10.
    to_base     A list of base 10 numbers to convert from_num to.  Base
                must be between 2 and 32 inclusive.

FLAGS:
    -b, --bare              Disable Pretty Print
    -c, --copy              Copy the resulting ouput to clipboard
        --from-clipboard    Get the input number from clipboard
    -h, --help              Prints help information
        --no-sep            Do not pad the output
    -s, --silent            Do not print output, for use with the clipboard on.
    -V, --version           Prints version information
    -v, --verbosity         Verbosity (more v's, more verbose)


OPTIONS:
    -f, --from-base <from-base>      Base of the input number [default: 10]
    -p, --pad <pad>                  Pad the output with leading 0s [default: 0]
        --sep-char <sep-char>        Specify spacer char [default: _]
    -l, --sep-length <sep-length>    Put a spacer every N characters [default: 4]
```


## Contributing
Before submitting a PR, please make sure you have:
* Run `cargo test`.
  * This runs all the defined tests.  Be sure to add test coverage for your PR when possible! 😁
* Checked there are no warnings when compiling.
  * `cargo clean && cargo build` ensures everything is compiled fresh and will give you maximum visibility on warnings.
  * TravisCI will be run with `cargo build --features=fail-on-warnings`, so give that a check to avoid waiting on the long CI build.
* Run `cargo fmt` and commit the result.
  * Some of the results when automatically formatting are not ideal (in my opinion), but a common format is better than inconsistent format (also in my opinion).  The default format is a safe decision to be consistent with the rest of the Rust community.  If there is a formatting decision you find untenable, open an Issue and start a discussion (please be civil, see the [Code of Conduct](CODE_OF_CONDUCT.md) 😉).

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
[Zach Nielsen](https://github.com/ZNielsen) - @ZNielsen
