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

#[cfg(test)]
mod tests {
    use crate::numbers::{round, round_currency, round_hours, round_long, round_percent, round_raw_hours, round_with_decimals, truncate};
    use rstest::rstest;

    #[rstest]
    #[case(0.0, 0)]
    #[case(0.1, 0)]
    #[case(0.49, 0)]
    #[case(0.5, 1)]
    #[case(0.51, 1)]
    #[case(0.99, 1)]
    #[case(1.0, 1)]
    #[case(1.1, 1)]
    #[case(1.49, 1)]
    #[case(1.5, 2)]
    #[case(1.51, 2)]
    fn test_round(#[case] value: f64, #[case] expected_result: i32) {
        assert_eq!(round(value), expected_result);
        assert_eq!(round_long(value), expected_result as i64);
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(0.1, 0.0)]
    #[case(0.49, 0.0)]
    #[case(0.5, 1.0)]
    #[case(0.51, 1.0)]
    #[case(0.99, 1.0)]
    #[case(1.0, 1.0)]
    #[case(1.1, 1.0)]
    #[case(1.49, 1.0)]
    #[case(1.5, 2.0)]
    #[case(1.51, 2.0)]
    fn test_round_0(#[case] value: f64, #[case] expected_result: f64) {
        assert_eq!(round_with_decimals(value, 0), expected_result);
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(0.1, 0.1)]
    #[case(0.499, 0.5)]
    #[case(0.5, 0.5)]
    #[case(0.501, 0.50)]
    #[case(0.505, 0.51)]
    #[case(0.99, 0.99)]
    #[case(0.9901, 0.99)]
    #[case(0.9905, 0.99)]
    #[case(0.991, 0.99)]
    #[case(0.995, 1.0)]
    fn test_round_2(#[case] value: f64, #[case] expected_result: f64) {
        assert_eq!(round_with_decimals(value, 2), expected_result);
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(0.1, 0.1)]
    #[case(0.499, 0.499)]
    #[case(0.5, 0.5)]
    #[case(0.501, 0.501)]
    #[case(0.505, 0.505)]
    #[case(0.99, 0.99)]
    #[case(0.9901, 0.9901)]
    #[case(0.9905, 0.9905)]
    #[case(0.991, 0.991)]
    #[case(0.995, 0.995)]
    fn test_round_percent(#[case] value: f64, #[case] expected_result: f64) {
        assert_eq!(round_percent(value), expected_result);
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(0.1, 0.1)]
    #[case(0.499, 0.499)]
    #[case(0.5, 0.5)]
    #[case(0.501, 0.501)]
    #[case(0.505, 0.505)]
    #[case(0.99, 0.99)]
    #[case(0.9901, 0.9901)]
    #[case(0.9905, 0.9905)]
    #[case(0.991, 0.991)]
    #[case(0.995, 0.995)]
    fn test_round_currency(#[case] value: f64, #[case] expected_result: f64) {
        assert_eq!(round_currency(value), expected_result);
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(0.1, 0.1)]
    #[case(0.499, 0.5)]
    #[case(0.5, 0.5)]
    #[case(0.501, 0.5)]
    #[case(0.505, 0.51)]
    #[case(0.99, 0.99)]
    #[case(0.9901, 0.99)]
    #[case(0.9905, 0.99)]
    #[case(0.991, 0.99)]
    #[case(0.995, 1.0)]
    fn test_round_hours(#[case] value: f64, #[case] expected_result: f64) {
        assert_eq!(round_hours(value), expected_result);
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(0.1, 0.1)]
    #[case(0.499, 0.499)]
    #[case(0.5, 0.5)]
    #[case(0.501, 0.501)]
    #[case(0.505, 0.505)]
    #[case(0.99, 0.99)]
    #[case(0.9901, 0.9901)]
    #[case(0.9905, 0.9905)]
    #[case(0.991, 0.991)]
    #[case(0.995, 0.995)]
    fn test_round_raw_hours(#[case] value: f64, #[case] expected_result: f64) {
        assert_eq!(round_raw_hours(value), expected_result);
    }

    #[rstest]
    #[case(0.0, 0)]
    #[case(0.1, 0)]
    #[case(0.49, 0)]
    #[case(0.5, 0)]
    #[case(0.51, 0)]
    #[case(0.99, 0)]
    #[case(1.0, 1)]
    #[case(1.1, 1)]
    #[case(1.49, 1)]
    #[case(1.5, 1)]
    #[case(1.51, 1)]
    fn test_truncate(#[case] value: f64, #[case] expected_result: i32) {
        assert_eq!(truncate(value), expected_result);
    }
}
