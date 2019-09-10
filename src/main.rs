use std::env;

const BASE_ARG_INDEX:   usize = 1;
const NUM_ARG_INDEX:    usize = 2;
const TARGET_ARG_INDEX: usize = 3;

struct Radix {
	x: i128,
	radix: u32
}

impl Radix {
	fn new(x: i128, radix: u32) -> Self {
		Self{x, radix}
	}

	fn as_str() -> str {
		// Print the number `x` with the base of `radix`
		let str_num = String::new();
	}
}


impl std::fmt::Display for Radix {
	write!("{:?}", self.as_str());
}

fn main() {
    // Get args 
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_help();
        return;
    }
    let base = &args[BASE_ARG_INDEX];
    let num  = &args[NUM_ARG_INDEX];
    let mut target_base;

    if args.len() == 4 {
        // Target base to be copied to clipboard
        target_base = &args[TARGET_ARG_INDEX];
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
    let num = match isize::from_str_radix(&num, base) {
        Ok(v)  => v,
        Err(_e) => {
            println!("Could not convert {} from base {}", num, base);
            print_help();
            return;
        },
    };

    // Print conversions
    println!("TODO: print all conversions");
	
}

fn print_help() {
    println!("TODO: make the help menu");
}
