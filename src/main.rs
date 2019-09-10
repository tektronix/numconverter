use std::{env, convert::TryInto};

const BASE_ARG_INDEX:   usize = 1;
const NUM_ARG_INDEX:    usize = 2;
const TARGET_ARG_INDEX: usize = 3;


//impl std::fmt::Display for Radix {
    //write!("{:?}", self.as_str());
//}

fn main() {
    // Get args
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_help();
        return;
    }
    let base = &args[BASE_ARG_INDEX];
    let num  = &args[NUM_ARG_INDEX];

    let mut target_base: Option<&str> = None;

    if args.len() == 4 {
        // Target base to be copied to clipboard
        target_base = Some(args[TARGET_ARG_INDEX].as_str());
    }


    // Parse base
    let base: u32 = match base.as_str() {
        "b" | "bin" | "binary"      => 2,
        "o" | "oct" | "octal"       => 8,
        "d" | "dec" | "decimal"     => 10,
        "h" | "hex" | "hexadecimal" => 16,
        _ => {
            println!("Error matching base {:?}", base);
            print_help();
            return;
        },
    };


    // Convert
    let num = match i128::from_str_radix(&num, base) {
        Ok(v)  => v,
        Err(_e) => {
            println!("Could not convert {} from base {}", num, base);
            print_help();
            return;
        },
    };

    // Print conversions
    println!("Bin: {}", as_string_base(&num,  2).unwrap().as_str());
    println!("Oct: {}", as_string_base(&num,  8).unwrap().as_str());
    println!("Dec: {}", as_string_base(&num, 10).unwrap().as_str());
    println!("Hex: {}", as_string_base(&num, 16).unwrap().as_str());
    if target_base.is_some() {
        let custom_base = u32::from_str_radix(&target_base.unwrap(), 10).unwrap();
        println!("Base {:?}: {}", &custom_base, as_string_base(&num, custom_base).unwrap().as_str());
    }
}

fn as_string_base(num: &i128, base: u32) -> Result<String, &str> {
    if base<1 || base>32 {
        Err("Invalid Base.  Base must be between 1 and 32")
    }
    else {
        // Print the number `x` with the base of `radix`
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

fn print_help() {
    println!("TODO: make the help menu");
}
