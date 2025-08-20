pub mod numbers {
    const ROUND_MULTIPLIERS: [f64; 6] = [1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0];

    const CURRENCY_ROUND_PRECISION: usize = 4;
    const HOURS_ROUND_PRECISION: usize = 2;
    const RAW_HOURS_ROUND_PRECISION: usize = 4;
    const ROUND_PERCENT_PRECISION: usize = 4;

    pub fn round(value: f64) -> i32 {
        if value >= 0.0 {
            if value - (value as i32 as f64) >= 0.5 {
                value as i32 + 1
            } else {
                value as i32
            }
        } else {
            let temp_value = -value;
            let mut result = if temp_value - (temp_value as i32 as f64) >= 0.5 {
                temp_value as i32 + 1
            } else {
                temp_value as i32
            };
            result = -result;
            result
        }
    }

    pub fn round_long(value: f64) -> i64 {
        if value >= 0.0 {
            if value - (value as i64 as f64) >= 0.5 {
                value as i64 + 1
            } else {
                value as i64
            }
        } else {
            let temp_value = -value;
            let mut result = if temp_value - (temp_value as i64 as f64) >= 0.5 {
                temp_value as i64 + 1
            } else {
                temp_value as i64
            };
            result = -result;
            result
        }
    }

    pub fn round_with_decimals(value: f64, number_decimals: usize) -> f64 {
        if number_decimals == 0 {
            return value.round(); // Java’s Math.rint → Rust’s round()
        }

        let multiplier = ROUND_MULTIPLIERS[number_decimals];
        let signum = value.signum();

        // Hokie trick for floating point edge cases
        let rounded = ((value * multiplier) + signum * 0.01 / multiplier).round() / multiplier;

        // epsilon tolerance
        let epsilon = 10f64.powi(-(number_decimals as i32));
        if rounded <= (0.0 - epsilon) || rounded >= (0.0 + epsilon) {
            rounded
        } else {
            0.0
        }
    }

    pub fn round_percent(value: f64) -> f64 {
        round_with_decimals(value, ROUND_PERCENT_PRECISION)
    }

    pub fn round_currency(value: f64) -> f64 {
        round_with_decimals(value, CURRENCY_ROUND_PRECISION)
    }

    pub fn round_hours(value: f64) -> f64 {
        round_with_decimals(value, HOURS_ROUND_PRECISION)
    }

    pub fn round_raw_hours(value: f64) -> f64 {
        round_with_decimals(value, RAW_HOURS_ROUND_PRECISION)
    }

    pub fn truncate(value: f64) -> i32 {
        value as i32
    }
}
