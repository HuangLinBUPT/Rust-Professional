pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // Initialize with first two Fibonacci numbers
    let mut f1 = 1;
    let mut f2 = 1;
    let mut sum = 2;  // Both 1s are odd

    while f2 <= threshold {
        let next = f1 + f2;
        if next > threshold {
            break;
        }
        if next % 2 == 1 {
            sum += next;
        }
        f1 = f2;
        f2 = next;
    }

    sum
}
