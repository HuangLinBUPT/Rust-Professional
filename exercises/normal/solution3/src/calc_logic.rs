pub fn new_birthday_probability(n: u32) -> f64 {
    if n >= 366 {
        return 1.0;
    }

    let mut probability = 1.0;
    for i in 0..n {
        probability *= (365 - i) as f64 / 365.0;
    }

    // Round to 4 decimal places
    ((1.0 - probability) * 10000.0).round() / 10000.0
}
