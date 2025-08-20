use ordered_float::OrderedFloat;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Variance {
    value1: OrderedFloat<f64>,
    value2: OrderedFloat<f64>,
}

impl Variance {
    pub fn between(value1: f64, value2: f64) -> Self {
        Self {
            value1: OrderedFloat(value1),
            value2: OrderedFloat(value2),
        }
    }

    pub fn as_absolute(&self) -> f64 {
        self.value1.0 - self.value2.0
    }

    pub fn as_percent(&self) -> f64 {
        let v1 = self.value1.0;
        let v2 = self.value2.0;

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
