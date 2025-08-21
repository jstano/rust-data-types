use crate::daterange::DateRange;
use crate::dateutils::{add_years, subtract_years};
use chrono::{Duration, NaiveDate};

pub struct AnnualDateRange;

impl AnnualDateRange {
    pub fn with_start_date(start_date: NaiveDate) -> DateRange {
        let end_date = add_years(start_date, 1) - Duration::days(1);

        DateRange::new_with_prior_next(
            start_date,
            end_date,
            AnnualDateRange::prior,
            AnnualDateRange::next,
        )
    }

    pub fn with_end_date(end_date: NaiveDate) -> DateRange {
        let start_date = subtract_years(end_date, 1) + Duration::days(1);

        DateRange::new_with_prior_next(
            start_date,
            end_date,
            AnnualDateRange::prior,
            AnnualDateRange::next,
        )
    }

    /// Returns the previous year.
    pub fn prior(date_range: &DateRange) -> DateRange {
        let start = subtract_years(date_range.start_date(), 1);
        let end = subtract_years(date_range.end_date(), 1);

        DateRange::new_with_prior_next(
            start,
            end,
            AnnualDateRange::prior,
            AnnualDateRange::next,
        )
    }

    /// Returns the next year.
    pub fn next(date_range: &DateRange) -> DateRange {
        let start = add_years(date_range.start_date(), 1);
        let end = add_years(date_range.end_date(), 1);

        DateRange::new_with_prior_next(
            start,
            end,
            AnnualDateRange::prior,
            AnnualDateRange::next,
        )
    }
}
