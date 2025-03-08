pub fn retire_time(time: &str, tp: &str) -> String {
    // Parse birth date
    let parts: Vec<&str> = time.split('-').collect();
    let birth_year: i32 = parts[0].parse().unwrap();
    let birth_month: i32 = parts[1].parse().unwrap();

    // Get base retirement age
    let base_age = match tp {
        "原法定退休年龄50周岁女职工" => 50,
        "原法定退休年龄55周岁女职工" => 55,
        "男职工" => 60,
        _ => panic!("Invalid person type")
    };

    // Calculate delay months based on birth year and person type
    let delay_months = if birth_year < 1964 {
        0
    } else {
        let mut delay = match tp {
            "男职工" => {
                if birth_year >= 1995 {
                    36  // Full delay for men born in/after 1995
                } else if birth_year < 1966 {
                    // For 1965-12, should get 3 months delay
                    let total_months = (birth_year - 1964) * 12 + birth_month;
                    // One month delay for every 8 months after 1964
                    total_months / 8
                } else {
                    36
                }
            },
            _ => {  // Women
                if birth_year >= 1995 {
                    match base_age {
                        50 => 60,  // 50->55
                        55 => 36,  // 55->58
                        _ => 0
                    }
                } else if birth_year < 1966 {
                    let months_after_1964 = (birth_year - 1964) * 12 + birth_month - 1;
                    (months_after_1964 + 1) / 3  // Gradual increase
                } else {
                    4  // Small delay for women born between 1966-1994
                }
            }
        };
        delay
    };

    // Calculate retirement date
    let total_months = birth_month + delay_months;
    let month_overflow = (total_months - 1) / 12;
    let retire_month = if total_months % 12 == 0 { 12 } else { total_months % 12 };
    let retire_year = birth_year + base_age + month_overflow;

    // Calculate actual retirement age
    let age = base_age as f64 + (delay_months as f64 / 12.0);

    // Format age with or without decimals
    let age_str = if (age * 100.0).round() % 100.0 == 0.0 {
        format!("{}", age as i32)
    } else {
        format!("{:.2}", age)
    };

    format!("{}-{:02},{},{}", retire_year, retire_month, age_str, delay_months)
}
