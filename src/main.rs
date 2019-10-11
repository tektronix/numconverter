////////////////////////////////////////////////////////////////////////////////
//  Module:   main.rs
//
//  Copyright © 2019 Zachary Nielsen
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.
////////////////////////////////////////////////////////////////////////////////
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

////////////////////////////////////////////////////////////////////////////////
//  Included Modules
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
//  Namespaces
////////////////////////////////////////////////////////////////////////////////
use std::{convert::TryInto, string::ToString};
use structopt::StructOpt;

////////////////////////////////////////////////////////////////////////////////
//  CODE
////////////////////////////////////////////////////////////////////////////////

#[cfg(target_os = "linux")]
const CLIPBOARD_WAIT_TIMER: std::time::Duration = std::time::Duration::from_secs(1);

#[derive(PartialEq)]
enum ErrorCode {
    BaseConversionErr,
    TargetBaseErr,
    InputBaseErr,
    ClipboardErr,
}

impl std::fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ErrorCode::BaseConversionErr => "Base Conversion Error",
                ErrorCode::TargetBaseErr => "Target Base Error",
                ErrorCode::InputBaseErr => "Input Base Error",
                ErrorCode::ClipboardErr => "Clipboard access Error",
            }
        )
    }
}

fn main() -> Result<(), ErrorCode> {
    // Get args
    let opt = Opt::from_args();

    if opt.verbosity > 0 {
        println!("{:?}", opt);
    }

    //
    // Sort out the optional indexed argument
    //
    let mut to_bases: Vec<String> = opt.to_bases.clone();
    let bases = get_bases(&opt, &mut to_bases)?;
    let from_base: u32 = bases.0;
    let from_num = bases.1;

    if to_bases.is_empty() {
        to_bases = vec![
            "2".to_string(),
            "8".to_string(),
            "10".to_string(),
            "16".to_string(),
        ]
    }

    //
    // Convert input number to base 10
    //
    let num = convert_to_base_10(from_num, from_base, opt.sep_char)?;

    // Buffer to store content for the clipboard
    let mut clipboard_buffer = String::default();

    // Print conversions
    for target_base in to_bases {
        let custom_base = match u32::from_str_radix(&target_base, 10) {
            Ok(v) => v,
            Err(_) => {
                println!(
                    "Error with target base {}\nPlease provide target base is base 10.",
                    target_base
                );
                return Err(ErrorCode::TargetBaseErr);
            }
        };
        let mut out_str = match as_string_base(&num, custom_base) {
            Ok(v) => v,
            Err(e) => {
                println!("Error with custom base:\n\t{}", e);
                return Err(ErrorCode::InputBaseErr);
            }
        };

        if !opt.silent || opt.copy {
            if !opt.no_sep && opt.sep_length > 0 {
                // Pad string every opt.spacer_length characters
                // Need size-1/spacer_len additional slots in the string
                let mut insert_idx: i32 = out_str.len() as i32 - opt.sep_length as i32;
                while insert_idx > 0 {
                    let left = String::from(&out_str[..(insert_idx as usize)]);
                    let right = String::from(&out_str[(insert_idx as usize)..]);
                    out_str = left;
                    out_str.push(opt.sep_char);
                    out_str.push_str(&right);
                    insert_idx -= opt.sep_length as i32;
                }
            }
            if !opt.silent {
                if !opt.bare {
                    print!("Base {:02}: ", &custom_base);
                }
                println!("{}", out_str);
            }
            if opt.copy {
                if !opt.bare {
                    clipboard_buffer += &format!("Base {:02}: ", &custom_base);
                }
                clipboard_buffer += &format!("{}\n", out_str);
            }
        }
    }

    if opt.copy {
        handle_clipboard(clipboard_buffer)
    } else {
        Ok(())
    }
}

#[cfg(target_os = "linux")]
fn handle_clipboard(content: String) -> Result<(), ErrorCode> {
    use nix::unistd::{fork, ForkResult};
    use x11_clipboard::Clipboard;

    match fork() {
        Err(_) => Err(ErrorCode::ClipboardErr),
        Ok(ForkResult::Child) => {
            let clipboard = Clipboard::new()
                .map_err(|_e| ErrorCode::ClipboardErr)
                .unwrap();
            let conn = &clipboard.setter.connection;

            clipboard
                .store(
                    clipboard.setter.atoms.clipboard,
                    clipboard.setter.atoms.utf8_string,
                    content,
                )
                .unwrap();

            while let Ok(()) = conn.has_error() {
                if x11_clipboard::xcb::get_selection_owner(&conn, clipboard.setter.atoms.clipboard)
                    .get_reply()
                    .map(|reply| reply.owner() != clipboard.setter.window)
                    .unwrap_or(true)
                {
                    break;
                }
                std::thread::sleep(CLIPBOARD_WAIT_TIMER);
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

#[cfg(not(target_os = "linux"))]
fn handle_clipboard(content: String) -> Result<(), ErrorCode> {
    use clipboard::ClipboardProvider;

    let mut clipboard = clipboard::ClipboardContext::new().map_err(|_e| ErrorCode::ClipboardErr)?;
    clipboard
        .set_contents(content)
        .map_err(|_e| ErrorCode::ClipboardErr)
}

#[cfg(target_os = "linux")]
fn get_clipboard_content() -> Result<Option<String>, ErrorCode> {
    use x11_clipboard::Clipboard;

    let clipboard = Clipboard::new().map_err(|_| ErrorCode::ClipboardErr)?;
    let val = clipboard
        .load(
            clipboard.setter.atoms.clipboard,
            clipboard.setter.atoms.utf8_string,
            clipboard.setter.atoms.property,
            std::time::Duration::from_secs(3),
        )
        .map_err(|_| ErrorCode::ClipboardErr)?;
    let content = String::from_utf8(val).unwrap();

    Ok(Some(content.trim().to_string()))
}

#[cfg(not(target_os = "linux"))]
fn get_clipboard_content() -> Result<Option<String>, ErrorCode> {
    use clipboard::ClipboardProvider;

    let mut clipboard = clipboard::ClipboardContext::new().map_err(|_| ErrorCode::ClipboardErr)?;
    let content = clipboard.get_contents()?;
    Ok(Some(content.trim().to_string()))
}

////////////////////////////////////////////////////////////////////////////////
// NAME:   get_from_base
//
// NOTES:  Matches one of the valid input base chars to its number
// ARGS:   from_base - a single character representing the input base
// RETURN:
//     The base of the input or None if there was no match.  None either
//     indicates an error in the base, or the base was omitted.  Either way,
//     the result is handled downstream.
//
fn get_from_base(from_base: &str) -> Option<u32> {
    match from_base {
        "b" => Some(2),
        "o" => Some(8),
        "d" => Some(10),
        "h" | "x" => Some(16),
        _ => None,
    }
}

////////////////////////////////////////////////////////////////////////////////
// NAME:   get_bases
//
// NOTES:
//     Handles the shifting of arguments if there was no from_base_char
//     provided.
// ARGS:
//     opt - command line options
//     to_bases - the list of bases to convert to (possibly empty)
// RETURN:
//     A tuple - (from_base, from_num)
//         from_base - the base we are converting from
//         from_num  - the number to convert, given in base specified
//                     by from_base
//
fn get_bases(opt: &Opt, to_bases: &mut Vec<String>) -> Result<(u32, Option<String>), ErrorCode> {
    let from_base_char = opt.from_base_char.clone().unwrap_or("".to_string());
    match get_from_base(from_base_char.as_str()) {
        Some(v) => Ok((v, opt.from_num.clone())),
        None => {
            // No base_char. Push from_num to the bases Vec, push base_char to from_num.
            if let Some(a_base) = &opt.from_num {
                to_bases.insert(0, a_base.clone());
            }
            if !opt.from_clipboard {
                // base_char wasn't provided, use the `-b` flag value as the base.
                Ok((opt.from_base, Some(from_base_char)))
            } else {
                if from_base_char != "" {
                    to_bases.insert(0, from_base_char.clone());
                }
                // base_char wasn't provided, use the `-b` flag value as the base.
                // get from_num from clipboard
                let from_num = get_clipboard_content()?;
                Ok((opt.from_base, from_num))
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// NAME:   convert_to_base_10
//
// NOTES:  What it says on the tin
// ARGS:
//     from_num - String representation of a number in base `from_base
//     from_base - Base the input number is given in
//     sep_char - `from_base` String may have zero or more `sep_char` in it.
//                 The function must strip out those chars before converting.
// RETURN: Base in base 10, or an error.
//
fn convert_to_base_10(
    from_num: Option<String>,
    from_base: u32,
    sep_char: char,
) -> Result<u128, ErrorCode> {
    let from_num = if let Some(num) = from_num {
        num.replace(sep_char, "")
    } else {
        println!("no number to convert was provided");
        return Err(ErrorCode::InputBaseErr);
    };

    match u128::from_str_radix(&from_num, from_base) {
        Ok(v) => Ok(v),
        Err(_e) => {
            println!("Could not convert {} from base {}", from_num, from_base);
            return Err(ErrorCode::BaseConversionErr);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// NAME:   as_string_base
//
// NOTES:  Converts the number `num` to a string representation of base `base`
// ARGS:
//     num - input number
//     base - output base
// RETURN: The number as a string, or an error
//
fn as_string_base(num: &u128, base: u32) -> Result<String, String> {
    if base < 2 || base > 33 {
        Err(String::from(
            "Invalid Base.  Base must be between 2 and 32 inclusive",
        ))
    } else {
        let mut str_num = String::new();

        let mut tmp: u128 = *num;
        let mut count: u32 = 0;

        while tmp > 0 {
            let radix_mask: u128 = u128::from((base as u128).pow(count));
            let digit: u8 = match ((tmp / radix_mask) % u128::from(base)).try_into() {
                Ok(v) => v,
                Err(_) => {
                    return Err(format!("Error while trying to convert to radix {}", base));
                }
            };

            let ch = if digit >= 10 {
                (b'A' + (digit - 10)) as char
            } else {
                (b'0' + digit) as char
            };

            str_num = ch.to_string() + str_num.as_str();

            count += 1;
            tmp -= u128::from(digit) * radix_mask;
        }

        Ok(str_num)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "numconverter",
    about = "A CLI number conversion utility written in Rust"
)]
struct Opt {
    /// Pad the output with leading 0s
    #[structopt(short, long, default_value = "0")]
    pad: u8,

    /// Put a spacer every N characters
    #[structopt(short = "-l", long, default_value = "4")]
    sep_length: u32,

    /// Specify spacer char
    #[structopt(long, default_value = "_")]
    sep_char: char,

    /// Do not pad the output
    #[structopt(long)]
    no_sep: bool,

    /// Input Base
    ///
    /// base_char takes precedence over this setting
    #[structopt(short, long, default_value = "10")]
    from_base: u32,

    /// Do not print output (for use with clipboard)
    #[structopt(short, long)]
    silent: bool,

    /// Copy the resulting output to clipboard
    #[structopt(short, long)]
    copy: bool,

    /// Get the input number from clipboard
    #[structopt(long)]
    from_clipboard: bool,

    /// Disable Pretty Print
    #[structopt(long)]
    bare: bool,

    /// Verbosity (more v's, more verbose)
    #[structopt(short, long, parse(from_occurrences))]
    verbosity: u8,

    /// Char representation of input base (b, o, d, or h) [optional]
    from_base_char: Option<String>,

    /// Number to convert [optional. Use --from-clipboard to get num from clipboard]
    from_num: Option<String>,

    /// Bases to convert to
    to_bases: Vec<String>,
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin() {
        assert_eq!(as_string_base(&4,   2).unwrap(), "100");
        assert_eq!(as_string_base(&12,  2).unwrap(), "1100");
        assert_eq!(as_string_base(&187, 2).unwrap(), "10111011");
        assert_eq!(as_string_base(&69,  2).unwrap(), "1000101");
    }

    #[test]
    fn test_oct() {
        assert_eq!(as_string_base(&4,   8).unwrap(), "4");
        assert_eq!(as_string_base(&12,  8).unwrap(), "14");
        assert_eq!(as_string_base(&187, 8).unwrap(), "273");
        assert_eq!(as_string_base(&69,  8).unwrap(), "105");
    }

    #[test]
    fn test_hex() {
        assert_eq!(as_string_base(&4,   16).unwrap(), "4");
        assert_eq!(as_string_base(&12,  16).unwrap(), "C");
        assert_eq!(as_string_base(&187, 16).unwrap(), "BB");
        assert_eq!(as_string_base(&69,  16).unwrap(), "45");
    }

    #[test]
    fn test_get_bases() {
        let mut opt = Opt {
            pad: 0,
            sep_length: 4,
            sep_char: '_',
            no_sep: false,
            from_base: 10,
            silent: false,
            copy: false,
            from_clipboard: false,
            bare: false,
            verbosity: 0,
            from_base_char: Some("b".to_owned()),
            from_num: Some("187".to_owned()),
            to_bases: Vec::new(),
        };

        let mut to_bases: Vec<String> = opt.to_bases.clone();
        let res = get_bases(&opt, &mut to_bases).unwrap();
        assert_eq!(res.0, 2);
        assert_eq!(res.1, Some("187".to_owned()));
        assert!(to_bases.is_empty());

        opt.from_base_char = Some("80".to_owned());
        let res = get_bases(&opt, &mut to_bases).unwrap();
        assert_eq!(res.0, 10);
        assert_eq!(res.1, Some("80".to_owned()));
        assert!(!to_bases.is_empty());
    }

    #[test]
    fn test_convert_to_base_10() {
        assert_eq!(
            convert_to_base_10(Some("10111011".to_owned()), 2, '_'),
            Ok(187)
        );
        assert_eq!(convert_to_base_10(Some("273".to_owned()), 8,  '_'), Ok(187));
        assert_eq!(convert_to_base_10(Some("187".to_owned()), 10, '_'), Ok(187));
        assert_eq!(convert_to_base_10(Some("BB" .to_owned()), 16, '_'), Ok(187));
        assert_eq!(
            convert_to_base_10(None, 10, '_'),
            Err(ErrorCode::InputBaseErr)
        );
    }
}
