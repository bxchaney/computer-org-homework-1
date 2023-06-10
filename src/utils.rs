const HEX_ARR: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

/* Question 1: implementing toUpper and toLower with bitwise operations.

*/
pub fn to_upper(s: &String) -> String {
    let mut str = String::with_capacity(s.len());
    for c in s.chars() {
        if c >= 'a' && c <= 'z' {
            str.push(char::from(c as u8 & 0xdf));
        } else {
            str.push(c);
        }
    }

    return str;
}

pub fn to_lower(s: &String) -> String {
    let mut str = String::with_capacity(s.len());
    for c in s.chars() {
        if c >= 'A' && c <= 'Z' {
            str.push(char::from(c as u8 | 0x20));
        } else {
            str.push(c);
        }
    }

    return str;
}

/* Question 2: Converting string repr of binary number to hexidecimal

*/
pub fn bin_to_hex(s: &String) -> String {
    let r = s.len() % 4;
    let iterations = s.len() / 4;

    let mut out_str = String::with_capacity(iterations + 2);
    out_str.push('0');
    out_str.push('x');
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

    for i in 0..iterations {
        let slice = &s[(4 * i) + r..(4 * (i + 1) + r)];
        let mut j = 1;
        let mut v = 0;
        for c in slice.chars().rev() {
            if c == '1' {
                v += j;
            }
            j <<= 1;
        }
        out_str.push(HEX_ARR[v]);
    }

    return out_str;
}

/* Question 3: Converting integer to hexidecimal by masking 4 bits at a time */
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
