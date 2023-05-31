pub fn str_to_int(s: &String) -> i32 {
    
    let mut num:i32 = 0;
    let mut negative = false;
    for  c in s.chars() {
        
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

    return if negative {-1 * num} else  {num};
}
