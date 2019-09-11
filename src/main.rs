use std::{convert::TryInto, string::ToString};
use structopt::StructOpt;

fn main() {
    // Get args
    let opt = Opt::from_args();
    println!("{:?}", opt);

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
                println!("Base {:02}: {}", &custom_base, v);
            }
            Err(e) => {
                println!("Error with custom base: {}", e);
            }
        }
    }
}

fn as_string_base(num: &i128, base: u32) -> Result<String, &str> {
    if base<1 || base>32 {
        Err("Invalid Base.  Base must be between 1 and 32")
    }
    else {
        let mut str_num = String::new();

        let mut tmp: i128 = *num;
        let mut count: u32 = 0;

        while tmp > 0 {
            let radix_mask: i128 = i128::from(base.pow(count));
            let digit: u8 = ((tmp / radix_mask) % i128::from(base)).try_into().unwrap();

            // println!("count: {}, tmp: {}, radix_mask: {}, digit: {}", count, tmp, radix_mask, digit);

            // TODO: Rust must have a better way of dealing with ASCII stuff, right?
            let ch = match digit {
                    0  => "0",
                    1  => "1",
                    2  => "2",
                    3  => "3",
                    4  => "4",
                    5  => "5",
                    6  => "6",
                    7  => "7",
                    8  => "8",
                    9  => "9",
                    10 => "A",
                    11 => "B",
                    12 => "C",
                    13 => "D",
                    14 => "E",
                    15 => "F",
                    16 => "G",
                    17 => "H",
                    18 => "I",
                    19 => "J",
                    20 => "K",
                    21 => "L",
                    22 => "M",
                    23 => "N",
                    24 => "O",
                    25 => "P",
                    26 => "Q",
                    27 => "R",
                    28 => "S",
                    29 => "T",
                    30 => "U",
                    31 => "V",
                    _  => "Z",
            };

            str_num = ch.to_owned() + str_num.as_str();

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