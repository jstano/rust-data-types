use crate::daterange::{DateRange, DefaultDateRange};
use chrono::{NaiveDate};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArbitraryDateRange(DefaultDateRange);

impl ArbitraryDateRange {
    pub fn date(date: NaiveDate) -> Self {
        Self(DefaultDateRange::new(date, date))
    }

    pub fn of(start_date: NaiveDate, end_date: NaiveDate) -> Self {
        Self(DefaultDateRange::new(start_date, end_date))
    }
}

impl DateRange for ArbitraryDateRange {
    fn start_date(&self) -> NaiveDate {
        self.0.start_date()
    }

    fn end_date(&self) -> NaiveDate {
        self.0.end_date()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn create_new_date_range(&self, start: NaiveDate, end: NaiveDate) -> Self {
        ArbitraryDateRange(DefaultDateRange::new(start, end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, NaiveDate};

    #[test]
    fn test_single_date_range() {
        let date = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap();
        let range = ArbitraryDateRange::date(date);

        assert_eq!(range.start_date(), date);
        assert_eq!(range.end_date(), date);
        assert_eq!(range.len(), 1);
        assert!(range.contains_date(date));
        assert_eq!(range.dates(), vec![date]);
    }

    #[test]
    fn test_multi_date_range() {
        let start = NaiveDate::from_ymd(2025, 8, 1);
        let end = NaiveDate::from_ymd(2025, 8, 15);
        let range = ArbitraryDateRange::of(start, end);

        assert_eq!(range.start_date(), start);
        assert_eq!(range.end_date(), end);
        assert_eq!(range.len(), 15);
        assert!(range.contains_date(start));
        assert!(range.contains_date(end));
        assert!(!range.contains_date(NaiveDate::from_ymd(2025, 7, 31)));

        let dates = range.dates();
        assert_eq!(dates.first().unwrap(), &start);
        assert_eq!(dates.last().unwrap(), &end);
    }

    #[test]
    fn test_prior_and_next() {
        let start = NaiveDate::from_ymd(2025, 8, 1);
        let end = NaiveDate::from_ymd(2025, 8, 5);
        let range = ArbitraryDateRange::of(start, end);

        let prior = range.prior();
        assert_eq!(prior.start_date(), start - Duration::days(5));
        assert_eq!(prior.end_date(), end - Duration::days(5));

        let next = range.next();
        assert_eq!(next.start_date(), start + Duration::days(5));
        assert_eq!(next.end_date(), end + Duration::days(5));
    }

    #[test]
    fn test_ranges_before_after() {
        let start = NaiveDate::from_ymd(2025, 8, 1);
        let end = NaiveDate::from_ymd(2025, 8, 3);
        let range = ArbitraryDateRange::of(start, end);

        let before = range.ranges_before(2);
        assert_eq!(before.len(), 2);
        assert_eq!(before[1].end_date(), range.start_date() - Duration::days(1));

        let after = range.ranges_after(2);
        assert_eq!(after.len(), 2);
        assert_eq!(after[0].start_date(), range.end_date() + Duration::days(1));
    }
}
