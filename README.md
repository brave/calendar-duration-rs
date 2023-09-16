# calendar-duration

[![Crates.io](https://img.shields.io/crates/v/calendar-duration?style=for-the-badge)](https://crates.io/crates/calendar-duration)
[![docs.rs](https://img.shields.io/docsrs/calendar-duration?style=for-the-badge)](https://docs.rs/calendar-duration)
[![GitHub](https://img.shields.io/github/license/brave-experiments/calendar-duration?style=for-the-badge)](https://github.com/brave-experiments/calendar-duration/blob/master/LICENSE)

A Rust library containing a calendar respecting duration that is compatible with the `time` crate.
Supports parsing and displaying to/from strings. Also supports addition and subtraction with `OffsetDateTime`.

## Usage

Add the following dependency:

```
calendar-duration = "<latest version>"
```

See [documentation](https://docs.rs/calendar-duration) for details.

## Time string syntax

- `y` for years
- `mon` for months
- `w` for weeks
- `d` for days
- `h` for hours
- `m` for minutes
- `s` for seconds

The string can be prefixed with `-` for negative durations.

## Examples

- `1y3mon4d`
- `-3w4m5s`
