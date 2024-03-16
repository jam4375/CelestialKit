mod month;
pub mod timedelta;
mod utils;

pub use timedelta::TimeDelta;

/// Represents an instant in time
pub struct DateTime {
    /// Duration since the implicit epoch of 0001-01-01 00:00:00 TAI
    duration: TimeDelta,
}

impl DateTime {
    pub fn new(
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: f64,
    ) -> Option<Self> {
        if !utils::is_valid_year_month_day(year, month, day)
            || hour < 0
            || hour > 23
            || minute < 0
            || minute > 59
            || second < 0.0
            || second >= 60.0
        {
            return None;
        }

        let prev_year = year - 1;
        let doy = utils::day_of_year(year, month, day)?;
        let abs_days =
            doy - 1 + 365 * prev_year + prev_year / 4 - prev_year / 100 + prev_year / 400;

        Some(DateTime {
            duration: TimeDelta::new(
                abs_days as i128 * timedelta::NANOSECONDS_PER_DAY
                    + hour as i128 * timedelta::NANOSECONDS_PER_HOUR
                    + minute as i128 * timedelta::NANOSECONDS_PER_MINUTE
                    + (second * timedelta::NANOSECONDS_PER_SECOND as f64) as i128,
            ),
        })
    }
}
