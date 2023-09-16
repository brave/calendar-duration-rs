use std::ops::{Add, Neg, Sub};

use time::{Duration, OffsetDateTime};

use crate::CalendarDuration;

fn add_years(mut time: OffsetDateTime, years: i32) -> OffsetDateTime {
    loop {
        match time.replace_year(time.year() + years) {
            Err(_) => {
                assert_ne!(time.day(), 1);
                // year replacement failed because current day does not exist
                // in month of replaced year (i.e. February).
                // decrement current day and try again.
                time = time
                    .replace_day(time.day() - 1)
                    .expect("should be able to decrement day, in order in increment month");
            }
            Ok(new_time) => return new_time,
        }
    }
}

fn add_months(mut time: OffsetDateTime, months: i16) -> OffsetDateTime {
    // Get integer for current month, using 0 as the base,
    // and calculate the delta between the current month and target month
    let curr_month_zero_base = time.month() as i16 - 1;
    let total_month_delta = curr_month_zero_base + months;
    // Calculate the change in years from the month delta
    // by using the quotient of a division by 12 months.
    let mut years_delta = total_month_delta / 12;
    if total_month_delta.is_negative() {
        // If the time change is negative, we need to
        // subtract 1 from the year delta for correctness.
        years_delta -= 1;
    }
    if years_delta != 0 {
        // Adjust the year before adjusting the month, if necessary.
        time = add_years(time, years_delta.into());
    }
    let target_month = if months < 0 {
        time.month().nth_prev(months.unsigned_abs() as u8)
    } else {
        time.month().nth_next(months as u8)
    };
    loop {
        match time.replace_month(target_month) {
            Err(_) => {
                assert_ne!(time.day(), 1);
                // month replacement failed because current day does not exist
                // in replaced month. decrement current day and try again.
                time = time
                    .replace_day(time.day() - 1)
                    .expect("should be able to decrement day, in order in increment month");
            }
            Ok(new_time) => return new_time,
        }
    }
}

impl Neg for CalendarDuration {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.negative = !self.negative;
        self
    }
}

impl Add<CalendarDuration> for OffsetDateTime {
    type Output = OffsetDateTime;

    fn add(mut self, rhs: CalendarDuration) -> Self {
        if rhs.years > 0 {
            self = add_years(self, rhs.actual_unit_val(rhs.years.into()));
        }
        if rhs.months > 0 {
            self = add_months(self, rhs.actual_unit_val(rhs.months.into()));
        }
        if rhs.weeks > 0 {
            self += rhs.actual_unit_val(Duration::weeks(rhs.weeks.into()));
        }
        if rhs.days > 0 {
            self += rhs.actual_unit_val(Duration::days(rhs.days.into()));
        }
        if rhs.hours > 0 {
            self += rhs.actual_unit_val(Duration::hours(rhs.hours.into()));
        }
        if rhs.minutes > 0 {
            self += rhs.actual_unit_val(Duration::minutes(rhs.minutes.into()));
        }
        if rhs.seconds > 0 {
            self += rhs.actual_unit_val(Duration::seconds(rhs.seconds.into()));
        }
        self
    }
}

impl Sub<CalendarDuration> for OffsetDateTime {
    type Output = OffsetDateTime;

    fn sub(self, rhs: CalendarDuration) -> Self {
        self + -rhs
    }
}

#[cfg(test)]
mod tests {
    use time::{format_description::well_known::Rfc3339, OffsetDateTime};

    use crate::CalendarDuration;

    fn assert_rfc3339_eq(time: OffsetDateTime, formatted: &str) {
        assert_eq!(time.format(&Rfc3339).unwrap(), formatted);
    }

    #[test]
    fn basic_addition() {
        let base_time = OffsetDateTime::parse("2021-02-01T00:00:00Z", &Rfc3339).unwrap();

        let duration1 = CalendarDuration {
            hours: 1,
            minutes: 20,
            seconds: 5,
            ..Default::default()
        };
        assert_rfc3339_eq(base_time + duration1, "2021-02-01T01:20:05Z");

        let duration2 = CalendarDuration {
            months: 2,
            days: 5,
            hours: 5,
            seconds: 51,
            ..Default::default()
        };
        assert_rfc3339_eq(base_time + duration2, "2021-04-06T05:00:51Z");

        let duration3 = CalendarDuration {
            months: 1,
            weeks: 2,
            days: 1,
            hours: 1,
            ..Default::default()
        };
        assert_rfc3339_eq(base_time + duration3, "2021-03-16T01:00:00Z");

        let mut duration3 = CalendarDuration {
            days: 1,
            hours: 1,
            ..Default::default()
        };
        assert_rfc3339_eq(base_time - duration3, "2021-01-30T23:00:00Z");

        duration3 = -duration3;
        assert_rfc3339_eq(base_time + duration3, "2021-01-30T23:00:00Z");
    }

    #[test]
    fn february_addition() {
        let time = OffsetDateTime::parse("2020-01-31T00:00:00Z", &Rfc3339).unwrap();

        let month_duration = CalendarDuration {
            months: 1,
            ..Default::default()
        };
        let time = time + month_duration;
        assert_rfc3339_eq(time, "2020-02-29T00:00:00Z");

        let year_duration = CalendarDuration {
            years: 1,
            ..Default::default()
        };
        assert_rfc3339_eq(time + year_duration, "2021-02-28T00:00:00Z");
        assert_rfc3339_eq(time - year_duration, "2019-02-28T00:00:00Z");

        let time = OffsetDateTime::parse("2020-02-29T00:00:00Z", &Rfc3339).unwrap();
        let twelve_months_duration = CalendarDuration {
            months: 12,
            ..Default::default()
        };
        assert_rfc3339_eq(time + twelve_months_duration, "2021-02-28T00:00:00Z");
    }

    #[test]
    fn month_year_rollover() {
        let time = OffsetDateTime::parse("2021-12-15T11:00:00Z", &Rfc3339).unwrap();

        let mut duration = CalendarDuration {
            months: 1,
            ..Default::default()
        };
        let time = time + duration;
        assert_rfc3339_eq(time, "2022-01-15T11:00:00Z");

        duration.negative = true;
        assert_rfc3339_eq(time + duration, "2021-12-15T11:00:00Z");

        let time = OffsetDateTime::parse("2021-11-15T11:00:00Z", &Rfc3339).unwrap();

        let duration = CalendarDuration {
            months: 2,
            ..Default::default()
        };
        assert_rfc3339_eq(time + duration, "2022-01-15T11:00:00Z");

        let duration = CalendarDuration {
            months: 38,
            ..Default::default()
        };
        let time = time + duration;
        assert_rfc3339_eq(time, "2025-01-15T11:00:00Z");

        let time = time - duration;
        assert_rfc3339_eq(time, "2021-11-15T11:00:00Z");

        let duration = CalendarDuration {
            negative: true,
            months: 15,
            ..Default::default()
        };
        assert_rfc3339_eq(time + duration, "2020-08-15T11:00:00Z");

        let duration = CalendarDuration {
            months: 24,
            ..Default::default()
        };
        assert_rfc3339_eq(time + duration, "2023-11-15T11:00:00Z");
    }
}
