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
    pub fn from_gregorian(
        year: i128,
        month: i128,
        day: i128,
        hour: i128,
        minute: i128,
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

        // Compute number of integer days since the implicit epoch of 0001-01-01 00:00:00 TAI
        let prev_year = year - 1;
        let doy = utils::day_of_year(year, month, day)?;
        let abs_days =
            doy - 1 + 365 * prev_year + prev_year / 4 - prev_year / 100 + prev_year / 400;

        Some(DateTime {
            duration: TimeDelta::new(
                abs_days * timedelta::NANOSECONDS_PER_DAY
                    + hour * timedelta::NANOSECONDS_PER_HOUR
                    + minute * timedelta::NANOSECONDS_PER_MINUTE
                    + (second * timedelta::NANOSECONDS_PER_SECOND as f64) as i128,
            ),
        })
    }
}
