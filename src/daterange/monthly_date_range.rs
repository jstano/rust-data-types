use crate::daterange::DefaultDateRange;
use crate::daterange::DateRange;
use crate::dateutils::{add_months, last_day_of_month, subtract_months};
use chrono::{Datelike, Duration, Months, NaiveDate};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MonthlyDateRange {
    range: DefaultDateRange,
    start_day_of_month: u32,
}

impl MonthlyDateRange {
    pub fn with_end_date_on_first(end_date: NaiveDate) -> Self {
        Self::with_end_date_and_start_day(end_date, 1)
    }

    pub fn with_end_date_and_start_day(end_date: NaiveDate, start_day: u32) -> Self {
        let start = calculate_start_date_from_end_date(end_date, start_day);
        Self {
            range: DefaultDateRange::new(start, end_date),
            start_day_of_month: start_day,
        }
    }

    fn prior(&self) -> Self {
        if self.start_day_of_month == 1 {
            let new_end = self.range.start_date() - Duration::days(1);
            let new_start = new_end.with_day(1).unwrap();

            Self {
                range: DefaultDateRange::new(new_start, new_end),
                start_day_of_month: self.start_day_of_month,
            }
        } else {
            let new_start = subtract_months(self.range.start_date(), 1);
            let new_end = self.range.start_date() - Duration::days(1);

            Self {
                range: DefaultDateRange::new(new_start, new_end),
                start_day_of_month: self.start_day_of_month,
            }
        }
    }

    fn next(&self) -> Self {
        if self.start_day_of_month == 1 {
            let new_start = self.range.end_date() + Duration::days(1);
            let new_end = last_day_of_month(new_start);
            Self {
                range: DefaultDateRange::new(new_start, new_end),
                start_day_of_month: self.start_day_of_month,
            }
        } else {
            let new_start = self.range.end_date() + Duration::days(1);
            let new_end = add_months(self.range.end_date(), 1);

            Self {
                range: DefaultDateRange::new(new_start, new_end),
                start_day_of_month: self.start_day_of_month,
            }
        }
    }
}

impl DateRange for MonthlyDateRange {
    fn start_date(&self) -> NaiveDate {
        self.range.start_date()
    }

    fn end_date(&self) -> NaiveDate {
        self.range.end_date()
    }

    fn len(&self) -> usize {
        self.range.len()
    }

    fn create_new_date_range(&self, start: NaiveDate, end: NaiveDate) -> Self {
        Self {
            range: DefaultDateRange::new(start, end),
            start_day_of_month: self.start_day_of_month,
        }
    }
}

fn calculate_start_date_from_end_date(end_date: NaiveDate, start_day: u32) -> NaiveDate {
    if start_day == 1 {
        NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), 1).unwrap()
    } else {
        end_date
            .succ_opt()
            .unwrap() // plusDays(1)
            .checked_sub_months(Months::new(1))
            .unwrap()
    }
}
