/*
This program includes implemenations for all 4 functions described in Question
5 of Homework 1.

Function #1:

The functions to_upper and to_lower from utils.rs are implementations of
toUpper and toLower, respectively. If you call this program from the
command line and pass the flag "-a", then the program will perform
to_upper and to_lower on all other arguments passed to the command
line.

Function #2:

The function bin_to_hex from utils.rs converts a string representation of a
binary number integer to hexidexicaml. This function assumes that the input
string is only 1's and 0's. You may use this function by passing the flag "-b"
when the program is called from the command line. All other arguments passed
are assumed to be strings of 0's and 1's and the program will attempt to
convert to hexadecimal.

Function #3:

The function int_to_hex from utils.rs converts a signed 32-bit integer to its
hexadecimal representation. You may use this function by passing the flag "-c"
when the program is called from the command line. It is assumed that all other
command line arguments represent integers bounded by [-2^31, 2^31 - 1]. The
first parses the integers in the command line with Function #4, then it passes
the result to int_to_hex.

Function #4:

The function str_to_int from utils.rs converts a string of ASCII characters
representing an integer to a signed 32-bit value. You may use this function
by passing "-d" as a command line argument when the program is called. All
other command line arguments are assumed to be the string representations of
integers bounded by [-2^31, 2^31 - 1].

*/
mod utils;

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

    // Function #1
    if args.contains(&one_flag) {
        for s in inputs {
            if !s.eq(&one_flag) {
                println!("toUpper: {}", utils::to_upper(&s));
                println!("toLower: {}", utils::to_lower(&s));
            }
        }
        return;
    }

    // Function #2
    if args.contains(&two_flag) {
        for s in inputs {
            if !s.eq(&two_flag) {
                println!("{} -> {}", s, utils::bin_to_hex(&s));
            }
        }
        return;
    }

    // Function #3
    if args.contains(&three_flag) {
        for s in inputs {
            if !s.eq(&three_flag) {
                println!("{} -> {}", s, utils::int_to_hex(utils::str_to_int(&s)));
            }
        }
    }

    // Function #4
    if args.contains(&four_flag) {
        for s in inputs {
            if !s.eq(&four_flag) {
                println!("{} -> {}", s, utils::str_to_int(&s));
            }
        }
    }
}
