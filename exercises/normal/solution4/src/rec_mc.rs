const CASHES: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];

pub fn dp_rec_mc(amount: u32) -> u32 {
    if amount == 0 {
        return 0;
    }

    let mut dp = vec![u32::MAX; (amount + 1) as usize];
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in CASHES.iter() {
            if coin <= i {
                let prev = dp[(i - coin) as usize];
                if prev != u32::MAX {
                    dp[i as usize] = dp[i as usize].min(prev + 1);
                }
            }
        }
    }

    dp[amount as usize]
}
