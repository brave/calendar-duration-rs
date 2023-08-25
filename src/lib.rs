//! A library containing a calendar respecting duration that is compatible with the `time` crate.
//! Supports parsing and displaying to/from strings. Also supports addition and subtraction with `OffsetDateTime`.
//!
//! ## Time string syntax
//! - `y` for years
//! - `mon` for months
//! - `w` for weeks
//! - `d` for days
//! - `h` for hours
//! - `m` for minutes
//! - `s` for seconds
//!
//! The string can be prefixed with `-` for negative durations.
//!
//! ## Examples
//! - `1y3mon4d`
//! - `-3w4m5s`

mod ops;

use std::{
    fmt::{self, Display, Formatter},
    ops::Neg,
};

/// A calendar respecting duration structure.
#[derive(Debug, Copy, Clone, Default)]
pub struct CalendarDuration {
    pub negative: bool,
    pub years: u16,
    pub months: u8,
    pub weeks: u32,
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl CalendarDuration {
    fn set_unit_value(&mut self, value_str: &str, unit_str: &str) {
        match unit_str {
            "y" => self.years = value_str.parse().unwrap_or(0),
            "mon" => self.months = value_str.parse().unwrap_or(0),
            "w" => self.weeks = value_str.parse().unwrap_or(0),
            "d" => self.days = value_str.parse().unwrap_or(0),
            "h" => self.hours = value_str.parse().unwrap_or(0),
            "m" => self.minutes = value_str.parse().unwrap_or(0),
            "s" => self.seconds = value_str.parse().unwrap_or(0),
            _ => (),
        }
    }

    fn actual_unit_val<T: Neg<Output = T>>(&self, value: T) -> T {
        match self.negative {
            true => -value,
            false => value,
        }
    }

    /// Returns true if the duration is zero.
    pub fn is_zero(&self) -> bool {
        self.years == 0
            && self.months == 0
            && self.weeks == 0
            && self.days == 0
            && self.hours == 0
            && self.minutes == 0
            && self.seconds == 0
    }
}

fn format_unit_segment(segments: &mut Vec<String>, value: u32, unit: &str) {
    if value > 0 {
        segments.push(format!(
            "{} {}{}",
            value,
            unit,
            if value > 1 { "s" } else { "" }
        ));
    }
}

impl Display for CalendarDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut segments = Vec::new();
        format_unit_segment(&mut segments, self.years.into(), "year");
        format_unit_segment(&mut segments, self.months.into(), "month");
        format_unit_segment(&mut segments, self.weeks, "week");
        format_unit_segment(&mut segments, self.days, "day");
        format_unit_segment(&mut segments, self.hours, "hour");
        format_unit_segment(&mut segments, self.minutes, "minute");
        format_unit_segment(&mut segments, self.seconds, "second");
        if segments.len() >= 3 {
            // Produce a string with commas included. i.e. "1 hour, 2 minutes and 5 seconds"
            let combined_comma_segments = segments[0..(segments.len() - 1)].join(", ");
            write!(
                f,
                "{} and {}",
                combined_comma_segments,
                segments.last().unwrap()
            )
        } else {
            // If there are two or less segments, simply join the elements with 'and'.
            write!(f, "{}", segments.join(" and "))
        }
    }
}

impl From<&str> for CalendarDuration {
    fn from(s: &str) -> Self {
        let mut result = Self::default();
        let mut value_str = String::new();
        let mut unit_str = String::new();
        for (i, ch) in s.chars().enumerate() {
            if i == 0 && ch == '-' {
                result.negative = true;
            }
            if ch.is_alphabetic() {
                // If alphabetic, assume that we are examining the unit name.
                unit_str.push(ch);
            } else if ch.is_numeric() {
                // If numeric, assume that we are examining the numerical
                // value of the unit.
                // If the unit string is not empty, assume that we recorded
                // a value and unit previously which needs to be processed.
                if !unit_str.is_empty() {
                    result.set_unit_value(&value_str, &unit_str);
                    value_str.clear();
                    unit_str.clear();
                }
                value_str.push(ch);
            }
        }
        result.set_unit_value(&value_str, &unit_str);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::CalendarDuration;

    fn assert_parse_and_display_eq(formatted: &str, displayed: &str) {
        assert_eq!(CalendarDuration::from(formatted).to_string(), displayed);
    }

    #[test]
    fn parse_and_display() {
        assert_parse_and_display_eq("10s", "10 seconds");
        assert_parse_and_display_eq("40m1s", "40 minutes and 1 second");
        assert_parse_and_display_eq("1h20m41s", "1 hour, 20 minutes and 41 seconds");
        assert_parse_and_display_eq("5y2mon3w6h", "5 years, 2 months, 3 weeks and 6 hours");
    }

    #[test]
    fn parse_pos_neg() {
        let duration = CalendarDuration::from("15s");
        assert_eq!(duration.seconds, 15);
        assert!(!duration.negative);

        let duration = CalendarDuration::from("-10s");
        assert_eq!(duration.seconds, 10);
        assert!(duration.negative);
    }

    #[test]
    fn is_zero() {
        assert!(CalendarDuration::default().is_zero());
        assert!(!CalendarDuration {
            seconds: 1,
            ..Default::default()
        }
        .is_zero());
    }
}
