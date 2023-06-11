const HEX_ARR: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

/* Question 1: Implementing toUpper and toLower with bitwise operations.

   To convert any lowercase ASCII character to its corresponding uppercase
   representation, one only has to set its 3rd bit to 0. Conversely,
   to convert any uppercase ASCII character to its corresponding lowercase
   value, one has to set the 3rd bit to 1.

   When converting lower to upper, this is accomplished with a bitwise and
   operation using the mask 0b11011111 = 0xDF. The result will have the same
   bitwise values, save for the 3rd bit, which will now be 0.

   When converting upper to lower, this is accomplished with a bitwise or
   operation with the mask 0b00100000 = 0x20. The result will have the same
   bitwise values, save fo the 3rd bit, which will now be 1.

*/

/// to_upper assumes that s is a string of ASCII characters. The returned
/// String converts any characters between 'a' and 'z' to their corresponding
/// uppercase forms. All other characters are unchanged.
pub fn to_upper(s: &String) -> String {
    // Allocate new String with same number of characters as s
    let mut str = String::with_capacity(s.len());

    for c in s.chars() {
        if c >= 'a' && c <= 'z' {
            // Cast char to unsigned 8 bit integer to perform bitwise and,
            // this operation is only defined between integer types in Rust.
            // After bitwise and, create new char value from u8 value.
            // While it is true that some char values may be larger than u8,
            // this block only casts char's between 'a' and 'z' by construction.
            str.push(char::from(c as u8 & 0xDF));
        } else {
            // Push all other chars by default.
            str.push(c);
        }
    }

    return str;
}

/// to_lower assumes that s is a string of ASCII characters. The returned
/// String converts any characters between 'A' and 'Z' to their corresponding
/// lowercase forms. All other characters are unchanged.
pub fn to_lower(s: &String) -> String {
    // Allocate new String with same number of characters as s.
    let mut str = String::with_capacity(s.len());

    for c in s.chars() {
        if c >= 'A' && c <= 'Z' {
            // Cast char to unsigned 8 bit integer to perform bitwise or,
            // this operation is only defined between integer types in Rust.
            // After bitwise or, create new char value from u8 value.
            // While it is true that some char values may be larger than u8,
            // this block only casts char's between 'A' and 'Z' by construction.
            str.push(char::from(c as u8 | 0x20));
        } else {
            // Push all other chars by default.
            str.push(c);
        }
    }

    return str;
}

/* Question 2: Converting string repr of binary number to hexidecimal

*/

/// bin_to_hex assumes that the provided string is only a series of 0's and 1's.
/// This function returns the hexidecimal representation of provided string of
/// 0's and 1's.
pub fn bin_to_hex(s: &String) -> String {
    let r = s.len() % 4;
    let iterations = s.len() / 4;

    // If the number of characters in s is a multiple of 4, then
    // the total number of characters in the output string will be
    // 2 + iterations. Otherwise, because of truncation in integer
    // division, we have to add 1 to the capacity to account for
    // string lengths that are not multiples of 4.
    // This is not entirely necessary, Rust will allocate a new string
    // under the hood if we push characters to a string beyond its capacity,
    // but this prevents another string from being created.
    let mut out_str = String::with_capacity(
        iterations + 2 + {
            if r == 0 {
                0
            } else {
                1
            }
        },
    );
    out_str.push('0');
    out_str.push('x');

    // Handling leading bits when the string length is not a multiple of 4.
    // Read the leading r bits right to left to get the value of leading
    // hexadecimal digit.
    if r != 0 {
        let first_char = &s[0..r];
        let mut j = 1;
        let mut v = 0;
        for c in first_char.chars().rev() {
            if c == '1' {
                v += j;
            }
            j <<= 1;
        }
        out_str.push(HEX_ARR[v]);
    }

    // Read in the rest of the string 4 bits at a time. Read bits in each
    // slice from left to right to get the value of the next hexadecimal digit.
    for i in 0..iterations {
        let slice = &s[4 * i + r..(4 * (i + 1) + r)];
        let mut j = 0b1000;
        let mut v = 0;
        for c in slice.chars() {
            if c == '1' {
                v += j;
            }
            j >>= 1;
        }
        out_str.push(HEX_ARR[v]);
    }

    return out_str;
}

/* Question 3: Converting integer to hexadecimal by masking 4 bits at a time */
pub fn int_to_hex(i: i32) -> String {
    let mut nyble = 7;
    let mask = 0b1111;
    let mut s = String::with_capacity(10);
    s.push('0');
    s.push('x');

    // Most significant nyble of negative 2's compement integers is a special
    // case. Ignore the leading bit to allow right bitshifting to behave
    // as intended. Then, a bitwise union reintroduces the ignored leading bit.
    if i < 0 {
        // Shift 0b0111 28 bits to the left
        let leading_mask = 0x7 << (nyble * 4);

        // leading_nyble = 0b 0xxx 0000 0000 0000 0000 0000 0000 0000
        // where xxx are the 2nd, 3rd and 4th bits in i.
        let mut leading_nyble = i & leading_mask;

        // leading_nyble = 0b 0000 0000 0000 0000 0000 0000 0000 0xxx
        leading_nyble >>= nyble * 4;

        // leading_nyble = 0b 0000 0000 0000 0000 0000 0000 0000 1xxx
        leading_nyble |= 0b1000;

        // By construction, leading_nyble is at least 8 and at most F, base 16
        s.push(HEX_ARR[leading_nyble as usize]);
        nyble -= 1;
    }

    while nyble >= 0 {
        // shift 4-bit mask left to get bits in that window, then
        // shift it back to the right to get the character from the
        // lookup table HEX_ARR. By construction, c is at least 0 and
        // at most F, base 16, after these operations.
        let c = (i & (mask << (nyble * 4))) >> (nyble * 4);
        s.push(HEX_ARR[c as usize]);
        nyble -= 1;
    }

    return s;
}

/* Question 4: Converting ASCII representation of an integer to its integer
  value.
*/
pub fn str_to_int(s: &String) -> i32 {
    let mut num: i32 = 0;
    let mut negative = false;
    for c in s.chars() {
        // only expecting the first char to be '-', but this method
        // allows for strings of the form '000-000' to also be evaluated
        // as negative numbers.
        if c == '-' {
            negative = true;
            continue;
        }
        num *= 10;
        // Rust does not support char - char.
        // Have to cast values to i32 to perform subtraction
        num += c as i32 - '0' as i32;
    }

    return if negative { -1 * num } else { num };
}
