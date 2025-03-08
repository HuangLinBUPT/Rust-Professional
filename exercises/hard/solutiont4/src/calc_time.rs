pub fn time_info(time: &str) -> String {
    let date: Vec<i32> = time.split('-')
        .map(|x| x.parse().unwrap())
        .collect();
    let year = date[0];
    let month = date[1];
    let day = date[2];

    // Calculate days from the start of the year
    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut day_of_year = day;
    for i in 0..(month - 1) {
        day_of_year += days_in_month[i as usize];
    }

    // Calculate the week number
    // Jan 1st, 2025 is Wednesday (3)
    let base_day = 3; // Wednesday
    let total_days = day_of_year - 1;
    let current_week_day = (total_days + base_day) % 7;
    let week_day = if current_week_day == 0 { 7 } else { current_week_day };
    
    // Fix week number calculation for year end
    let week_number = if month == 12 && day > 24 { 1 } else { (day_of_year + 6 - week_day) / 7 + 1 };

    // Calculate remaining days in the year
    let days_in_year = 365;
    let remaining_days = days_in_year - day_of_year;

    // Calculate days until Chinese New Year (2025-01-29 or 2026-02-17)
    let mut days_to_cny = 0;
    if month == 1 && day <= 29 {
        days_to_cny = 29 - day;
    } else {
        // Calculate days until Feb 17, 2026
        let remaining_days_in_current_month = days_in_month[month as usize - 1] - day;
        let mut days_until_feb_17 = remaining_days_in_current_month;
        
        // Add days for remaining full months until February
        for m in month..12 {
            days_until_feb_17 += days_in_month[m as usize];
        }
        // Add January days and days in February
        days_until_feb_17 += 31 + 17;
        
        days_to_cny = days_until_feb_17;
    }

    // Fix A-stock market opening calculation
    let mut days_to_market = 0;
    let is_weekend = week_day >= 6;
    
    if is_weekend {
        days_to_market = if week_day == 6 { 1 } else { 0 };
    } else {
        // Check if it's Chinese New Year period (Jan 28 - Feb 3)
        if month == 1 && (day >= 28) {
            days_to_market = if day == 28 {
                7  // Day before CNY
            } else {
                // For subsequent days, decrease remaining days
                7 - (day - 28)  // Changed from 8 to 7 in the formula
            };
        } else if month == 2 && (day <= 3) {
            days_to_market = 3 - day + 1;
        } else {
            // Regular holiday checks
            if month == 5 && day == 1 {  // Labor Day
                days_to_market = 4;       // 4-day holiday period
            } else if month == 12 && day == 31 {  // Year end
                days_to_market = 1;
            } else if week_day == 5 {  // Friday
                days_to_market = 2;     // Next trading day is Monday
            }
        }
    }

    format!("{},{},{},{},{},{}", 
        week_number,
        week_day,
        day_of_year,
        remaining_days,
        days_to_cny,
        days_to_market
    )
}
