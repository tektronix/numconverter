use std::{convert::TryInto, string::ToString};
use structopt::StructOpt;

fn main() {
    // Get args
    let opt = Opt::from_args();

    if opt.verbosity > 0 {
        println!("{:?}", opt);
    }

    let bases: Vec<String> = if opt.bases.is_empty() {
        vec![
            "2" .to_string(),
            "8" .to_string(),
            "10".to_string(),
            "16".to_string()
        ]
    }
    else {
        opt.bases
    };

    // Convert
    let num = match i128::from_str_radix(&opt.num, opt.in_base) {
        Ok(v)  => v,
        Err(_e) => {
            println!("Could not convert {} from base {}", opt.num, opt.in_base);
            return;
        },
    };

    // Print conversions
    for target_base in bases {
        let custom_base = u32::from_str_radix(&target_base, 10).unwrap();
        match as_string_base(&num, custom_base) {
            Ok(v)  => {
                if !opt.silent {
                    if opt.pretty {
                        print!("Base {:02}: ", &custom_base);
                    }
                    println!("{}", v);
                }
            }
            Err(e) => {
                println!("Error with custom base: {}", e);
            }
        }
    }
}

fn as_string_base(num: &i128, base: u32) -> Result<String, &str> {
    if base<2 || base>33 {
        Err("Invalid Base.  Base must be between 1 and 33 (i.e. 2 to 32)")
    }
    else {
        let mut str_num = String::new();

        let mut tmp: i128 = *num;
        let mut count: u32 = 0;

        while tmp > 0 {
            let radix_mask: i128 = i128::from(base.pow(count));
            let digit: u8 = ((tmp / radix_mask) % i128::from(base)).try_into().unwrap();

            let ch = if digit >= 10 {
                (b'A' + (digit-10)) as char
            }
            else {
                (b'0' + digit) as char
            };

            str_num = ch.to_string() + str_num.as_str();

            count += 1;
            tmp -= i128::from(digit) * radix_mask;
        }

        Ok(str_num)
    }
}


#[derive(StructOpt, Debug)]
#[structopt(name = "numconverter", about = "A CLI number conversion utility written in Rust")]
struct Opt {
    /// Pad the output with leading 0s
    #[structopt(short, long, default_value = "0")]
    pad: u8,

    /// Input Base
    #[structopt(short, long, default_value = "10")]
    in_base: u32,

    /// Copy to system clipboard
    #[structopt(short, long)]
    copy: bool,

    /// Do not print output (for use with clipboard)
    #[structopt(short, long)]
    silent: bool,

    /// Pretty Print
    #[structopt(long)]
    pretty: bool,

    /// Verbosity (more v's, more verbose)
    #[structopt(short, long, parse(from_occurrences))]
    verbosity: u8,

    /// Number to convert
    num: String,

    /// Bases to convert to
    bases: Vec<String>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin() {

    }
}
