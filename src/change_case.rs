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
