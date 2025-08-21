#[derive(Debug, Clone, Copy)]
pub struct Variance {
    value1: f64,
    value2: f64,
}

impl Variance {
    pub fn between(value1: f64, value2: f64) -> Self {
        Self { value1, value2 }
    }

    pub fn as_absolute(&self) -> f64 {
        self.value1 - self.value2
    }

    pub fn as_percent(&self) -> f64 {
        let v1 = self.value1;
        let v2 = self.value2;

        if v1 != 0.0 && v2 != 0.0 && v1 != v2 {
            (v1 - v2) / v2 * 100.0
        } else if v1 == 0.0 && v2 == 0.0 {
            0.0
        } else if v1 == 0.0 {
            -100.0
        } else if v2 == 0.0 {
            100.0
        } else {
            0.0
        }
    }

    pub fn is_outside_allowed_variance_percentages(
        &self,
        allowed_from_variance_percentage: f64,
        allowed_to_variance_percentage: f64,
    ) -> bool {
        let percent_variance = self.as_percent();
        percent_variance < -allowed_from_variance_percentage
            || percent_variance > allowed_to_variance_percentage
    }
}

#[cfg(test)]
mod tests {
    use crate::variance::Variance;
    use rstest::rstest;

    #[rstest]
    #[case(10.0, 20.0, -10.0, -50.0, 25.0, 25.0, true)]
    #[case(10.0, 20.0, -10.0, -50.0, 49.9, 25.0, true)]
    #[case(10.0, 20.0, -10.0, -50.0, 50.0, 25.0, false)]
    #[case(10.0, 20.0, -10.0, -50.0, 50.1, 25.0, false)]
    #[case(20.0, 10.0, 10.0, 100.0, 25.0, 25.0, true)]
    #[case(20.0, 10.0, 10.0, 100.0, 25.0, 99.9, true)]
    #[case(20.0, 10.0, 10.0, 100.0, 25.0, 100.0, false)]
    #[case(20.0, 10.0, 10.0, 100.0, 25.0, 100.1, false)]
    #[case(0.0, 0.0, 0.0, 0.0, 25.0, 25.0, false)]
    #[case(0.0, 100.0, -100.0, -100.0, 25.0, 25.0, true)]
    #[case(100.0, 0.0, 100.0, 100.0, 25.0, 25.0, true)]
    #[case(100.0, 100.0, 0.0, 0.0, 25.0, 25.0, false)]
    fn test_variance_logic(
        #[case] value1: f64,
        #[case] value2: f64,
        #[case] absolute_variance: f64,
        #[case] percent_variance: f64,
        #[case] from_variance: f64,
        #[case] to_variance: f64,
        #[case] expected_outside_variance: bool,
    ) {
        let variance = Variance::between(value1, value2);

        assert_eq!(variance.as_absolute(), absolute_variance);
        assert_eq!(variance.as_percent(), percent_variance);
        assert_eq!(variance.is_outside_allowed_variance_percentages(from_variance, to_variance), expected_outside_variance);
    }
}
