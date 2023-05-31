mod change_case;
mod to_hex;
mod to_int;

use std::env;

fn main() {

    // Taking command line arguments
    let args: Vec<_> = env::args().collect();
    
    // ignoring the name of the executable
    let inputs = &args[1..];

    let one_flag = String::from("-a");
    let two_flag = String::from("-b");
    let three_flag = String::from("-c");
    let four_flag = String::from("-d");

    // Perform toUpper and toLower on the provided strings
    if args.contains(&one_flag) {
        for s in inputs {
            if !s.eq(&one_flag) {
                println!("toUpper: {}", change_case::to_upper(&s));
                println!("toLower: {}", change_case::to_lower(&s));
            }
        }
        return;
    }

    if args.contains(&two_flag) {
        for s in inputs {
            if !s.eq(&two_flag) {
                println!("{} -> {}", s, to_hex::bin_to_hex(&s));
            }
        }
        return;
    }

    if args.contains(&three_flag) {
        for s in inputs {
            if !s.eq(&three_flag) {
                let i = s.parse().unwrap();
                println!("{} -> {}", s, to_hex::int_to_hex(i));
            }
        }
    }

    if args.contains(&four_flag) {
        for s in inputs {
            if !s.eq(&four_flag) {
                println!("{} -> {}", s, to_int::str_to_int(s));
            }
        }
    }
}
