const HEX_ARR: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub fn bin_to_hex(s: &String) -> String {
    let r = s.len() % 4;
    let iterations = s.len() / 4;

    let mut out_str = String::with_capacity(iterations + 2);
    out_str.push('0');
    out_str.push('x');
    if r != 0 {
        let first_char = &s[0..r + 1];
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

pub fn int_to_hex(i: i128) -> String {
    let mut window = 31;
    let mask = 0b1111;
    let mut s = String::with_capacity(10);
    s.push('0');
    s.push('x');
    while window >= 0 {
        let c = (i & (mask << (window * 4))) >> (window * 4);
        println!("{}", c);
        s.push(HEX_ARR[c as usize]);
        window -= 1;
    }

    return s;
}
