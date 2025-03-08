pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // Parse the input string to get the number and its base
    let start = num_str.find('(').unwrap();
    let end = num_str.find(')').unwrap();
    let num = &num_str[..start];
    let from_base = num_str[start+1..end].parse::<u32>().unwrap();
    
    // Convert number from original base to decimal
    let decimal = u32::from_str_radix(num, from_base).unwrap();
    
    // Convert decimal to target base
    if decimal == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    let mut n = decimal;
    while n > 0 {
        let digit = n % to_base;
        let ch = if digit < 10 {
            (digit as u8 + b'0') as char
        } else {
            (digit as u8 - 10 + b'a') as char
        };
        result.insert(0, ch);
        n /= to_base;
    }
    result
}
