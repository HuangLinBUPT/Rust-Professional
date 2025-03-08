pub fn find_max_prime_factor(mut number: u128) -> u128 {
    if number <= 1 {
        return number;
    }

    let mut max_factor = 1;

    // Handle even numbers
    while number % 2 == 0 {
        max_factor = 2;
        number /= 2;
    }

    // Try small odd factors first
    let mut factor = 3;
    let limit = 1_000_000; // Limit for trial division
    
    while factor <= limit && factor * factor <= number {
        if number % factor == 0 {
            max_factor = factor;
            number /= factor;
            
            // Continue dividing by this factor
            while number % factor == 0 {
                number /= factor;
            }
        } else {
            factor += 2;
        }
    }

    // If the remaining number is greater than 1, it's either a prime
    // or has only large prime factors that we didn't check
    if number > 1 {
        max_factor = max_factor.max(number);
    }

    max_factor
}