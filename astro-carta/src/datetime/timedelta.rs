use std::fmt;
use std::ops;

// Constants
pub const NANOSECONDS_PER_MICROSECOND: i128 = 1000;
pub const NANOSECONDS_PER_MILLISECOND: i128 = 1000 * NANOSECONDS_PER_MICROSECOND;
pub const NANOSECONDS_PER_SECOND: i128 = 1000 * NANOSECONDS_PER_MILLISECOND;
pub const NANOSECONDS_PER_MINUTE: i128 = 60 * NANOSECONDS_PER_SECOND;
pub const NANOSECONDS_PER_HOUR: i128 = 60 * NANOSECONDS_PER_MINUTE;
pub const NANOSECONDS_PER_DAY: i128 = 24 * NANOSECONDS_PER_HOUR;

/// Represents a time difference or interval measured in nanoseconds.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeDelta {
    nanoseconds: i128,
}

impl TimeDelta {
    /// Constructs a new `TimeDelta` instance with the specified number of nanoseconds.
    ///
    /// # Arguments
    ///
    /// * `nanoseconds` - The number of nanoseconds for the time difference.
    ///
    /// # Returns
    ///
    /// A new `TimeDelta` instance initialized with the provided number of nanoseconds.
    pub fn new(nanoseconds: i128) -> Self {
        TimeDelta { nanoseconds }
    }

    /// Creates a new `TimeDelta` instance representing a certain number of days.
    ///
    /// # Arguments
    ///
    /// * `value` - A floating-point value representing the number of days.
    ///
    /// # Examples
    ///
    /// ```
    /// use astro_carta::datetime::timedelta;
    /// use astro_carta::datetime::TimeDelta;
    ///
    /// let delta = TimeDelta::days(3.5);
    /// assert_eq!(delta, TimeDelta::new(3 * timedelta::NANOSECONDS_PER_DAY + 12 * timedelta::NANOSECONDS_PER_HOUR))
    /// ```
    pub fn days(value: f64) -> Self {
        TimeDelta {
            nanoseconds: (value * NANOSECONDS_PER_DAY as f64) as i128,
        }
    }

    /// Creates a new `TimeDelta` instance representing a certain number of hours.
    ///
    /// # Arguments
    ///
    /// * `value` - A floating-point value representing the number of hours.
    ///
    /// # Examples
    ///
    /// ```
    /// use astro_carta::datetime::timedelta;
    /// use astro_carta::datetime::TimeDelta;
    ///
    /// let delta = TimeDelta::hours(8.5);
    /// assert_eq!(delta, TimeDelta::new(8 * timedelta::NANOSECONDS_PER_HOUR + 30 * timedelta::NANOSECONDS_PER_MINUTE))
    /// ```
    pub fn hours(value: f64) -> Self {
        TimeDelta {
            nanoseconds: (value * NANOSECONDS_PER_HOUR as f64) as i128,
        }
    }

    /// Creates a new `TimeDelta` instance representing a certain number of minutes.
    ///
    /// # Arguments
    ///
    /// * `value` - A floating-point value representing the number of minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use astro_carta::datetime::timedelta;
    /// use astro_carta::datetime::TimeDelta;
    ///
    /// let delta = TimeDelta::minutes(8.5);
    /// assert_eq!(delta, TimeDelta::new(8 * timedelta::NANOSECONDS_PER_MINUTE + 30 * timedelta::NANOSECONDS_PER_SECOND))
    /// ```
    pub fn minutes(value: f64) -> Self {
        TimeDelta {
            nanoseconds: (value * NANOSECONDS_PER_MINUTE as f64) as i128,
        }
    }

    /// Creates a new `TimeDelta` instance representing a certain number of seconds.
    ///
    /// # Arguments
    ///
    /// * `value` - A floating-point value representing the number of seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use astro_carta::datetime::timedelta;
    /// use astro_carta::datetime::TimeDelta;
    ///
    /// let delta = TimeDelta::seconds(8.5);
    /// assert_eq!(delta, TimeDelta::new(8 * timedelta::NANOSECONDS_PER_SECOND + 500 * timedelta::NANOSECONDS_PER_MILLISECOND))
    /// ```
    pub fn seconds(value: f64) -> Self {
        TimeDelta {
            nanoseconds: (value * NANOSECONDS_PER_SECOND as f64) as i128,
        }
    }

    /// Creates a new `TimeDelta` instance representing a certain number of milliseconds.
    ///
    /// # Arguments
    ///
    /// * `value` - A floating-point value representing the number of milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use astro_carta::datetime::timedelta;
    /// use astro_carta::datetime::TimeDelta;
    ///
    /// let delta = TimeDelta::milliseconds(8.5);
    /// assert_eq!(delta, TimeDelta::new(8 * timedelta::NANOSECONDS_PER_MILLISECOND + 500 * timedelta::NANOSECONDS_PER_MICROSECOND))
    /// ```
    pub fn milliseconds(value: f64) -> Self {
        TimeDelta {
            nanoseconds: (value * NANOSECONDS_PER_MILLISECOND as f64) as i128,
        }
    }

    /// Creates a new `TimeDelta` instance representing a certain number of microseconds.
    ///
    /// # Arguments
    ///
    /// * `value` - A floating-point value representing the number of microseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use astro_carta::datetime::timedelta;
    /// use astro_carta::datetime::TimeDelta;
    ///
    /// let delta = TimeDelta::microseconds(8.5);
    /// assert_eq!(delta, TimeDelta::new(8 * timedelta::NANOSECONDS_PER_MICROSECOND + 500))
    /// ```
    pub fn microseconds(value: f64) -> Self {
        TimeDelta {
            nanoseconds: (value * NANOSECONDS_PER_MICROSECOND as f64) as i128,
        }
    }

    fn days_component(&self) -> i128 {
        self.nanoseconds.abs() / NANOSECONDS_PER_DAY
    }

    fn hours_component(&self) -> i128 {
        (self.nanoseconds.abs() % NANOSECONDS_PER_DAY) / NANOSECONDS_PER_HOUR
    }

    fn minutes_component(&self) -> i128 {
        (self.nanoseconds.abs() % NANOSECONDS_PER_HOUR) / NANOSECONDS_PER_MINUTE
    }

    fn seconds_component(&self) -> i128 {
        (self.nanoseconds.abs() % NANOSECONDS_PER_MINUTE) / NANOSECONDS_PER_SECOND
    }

    fn nanoseconds_component(&self) -> i128 {
        self.nanoseconds.abs() % NANOSECONDS_PER_SECOND
    }
}

/// Implements the display trait for TimeDelta.
///
/// This allows formatting a `TimeDelta` struct to a string representation.
/// The format is as follows:
///
/// * If the time delta is negative, the string starts with a '-' sign.
/// * The days component is included if it is non-zero, followed by the hours, minutes,
///   seconds, and nanoseconds components.
///
/// # Examples
///
/// ```
/// use std::fmt::Display;
/// use astro_carta::datetime::timedelta;
/// use astro_carta::datetime::TimeDelta;
///
/// let days_component = 3;
/// let hours_component = 6;
/// let minutes_component = 43;
/// let seconds_component = 13;
/// let nanoseconds_component = 123_456_789;
///
/// let delta = TimeDelta::new(
///     days_component * timedelta::NANOSECONDS_PER_DAY
///         + hours_component * timedelta::NANOSECONDS_PER_HOUR
///         + minutes_component * timedelta::NANOSECONDS_PER_MINUTE
///         + seconds_component * timedelta::NANOSECONDS_PER_SECOND
///         + nanoseconds_component,
/// );
/// assert_eq!(delta.to_string(), "3 days 06:43:13.123456789");
/// ```
impl fmt::Display for TimeDelta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.nanoseconds.signum() < 0 {
            write!(f, "-")?;
        }

        let days_component = self.days_component();

        if days_component.abs() > 0 {
            write!(f, "{} day", days_component)?;
            if days_component > 1 {
                write!(f, "s")?;
            }

            write!(
                f,
                " {:02}:{:02}:{:02}.{:09}",
                self.hours_component(),
                self.minutes_component(),
                self.seconds_component(),
                self.nanoseconds_component()
            )
        } else {
            write!(
                f,
                "{:02}:{:02}:{:02}.{:09}",
                self.hours_component(),
                self.minutes_component(),
                self.seconds_component(),
                self.nanoseconds_component()
            )
        }
    }
}

/// Implements the addition operator (`+`) for `TimeDelta`.
///
/// This allows adding two `TimeDelta` instances together, resulting in a new `TimeDelta`.
///
/// # Parameters
///
/// - `self`: The left-hand side operand of the addition.
/// - `rhs`: The right-hand side operand of the addition.
///
/// # Returns
///
/// A new `TimeDelta` representing the sum of the two operands.
///
/// # Panics
///
/// This function does not panic.
///
/// # Examples
///
/// ```
/// use astro_carta::datetime::TimeDelta;
///
/// let td1 = TimeDelta::new(100);
/// let td2 = TimeDelta::new(200);
/// let result = td1 + td2;
/// assert_eq!(result, TimeDelta::new(300));
/// ```
impl ops::Add<TimeDelta> for TimeDelta {
    type Output = TimeDelta;

    fn add(self, rhs: TimeDelta) -> TimeDelta {
        TimeDelta {
            nanoseconds: self.nanoseconds + rhs.nanoseconds,
        }
    }
}

/// Implements the subtraction operator (`-`) for `TimeDelta`.
///
/// This allows subtracting one `TimeDelta` instance from another, resulting in a new `TimeDelta`.
///
/// # Parameters
///
/// - `self`: The left-hand side operand of the subtraction.
/// - `rhs`: The right-hand side operand of the subtraction.
///
/// # Returns
///
/// A new `TimeDelta` representing the difference between the two operands.
///
/// # Panics
///
/// This function does not panic.
///
/// # Examples
///
/// ```
/// use astro_carta::datetime::TimeDelta;
///
/// let td1 = TimeDelta::new(300);
/// let td2 = TimeDelta::new(100);
/// let result = td1 - td2;
/// assert_eq!(result, TimeDelta::new(200));
/// ```
impl ops::Sub<TimeDelta> for TimeDelta {
    type Output = TimeDelta;

    fn sub(self, rhs: TimeDelta) -> TimeDelta {
        TimeDelta {
            nanoseconds: self.nanoseconds - rhs.nanoseconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // Test creating a positive time delta
        let td_positive = TimeDelta::new(100);
        assert_eq!(td_positive.nanoseconds, 100);

        // Test creating a negative time delta
        let td_negative = TimeDelta::new(-100);
        assert_eq!(td_negative.nanoseconds, -100);

        // Test creating a timedelta with zero nanoseconds
        let td_zero = TimeDelta::new(0);
        assert_eq!(td_zero.nanoseconds, 0);
    }

    #[test]
    fn test_equality() {
        let td1 = TimeDelta::new(100);
        let td2 = TimeDelta::new(100);
        let td3 = TimeDelta::new(200);

        assert_eq!(td1, td1);
        assert_eq!(td1, td2);
        assert_ne!(td1, td3);
    }

    #[test]
    fn test_ordering() {
        let td1 = TimeDelta::new(100);
        let td2 = TimeDelta::new(200);
        let td3 = TimeDelta::new(300);

        assert!(td1 < td2);
        assert!(td2 > td1);
        assert!(td2 < td3);

        assert!(td1 <= td2);
        assert!(td2 >= td1);
        assert!(td2 <= td3);

        assert!(td1 <= td1);
        assert!(td1 >= td1);
    }

    #[test]
    fn test_components() {
        let days_component = 3;
        let hours_component = 6;
        let minutes_component = 43;
        let seconds_component = 13;
        let nanoseconds_component = 123_456_789;
        let td1 = TimeDelta::new(
            days_component * NANOSECONDS_PER_DAY
                + hours_component * NANOSECONDS_PER_HOUR
                + minutes_component * NANOSECONDS_PER_MINUTE
                + seconds_component * NANOSECONDS_PER_SECOND
                + nanoseconds_component,
        );

        assert_eq!(days_component, td1.days_component());
        assert_eq!(hours_component, td1.hours_component());
        assert_eq!(minutes_component, td1.minutes_component());
        assert_eq!(seconds_component, td1.seconds_component());
        assert_eq!(nanoseconds_component, td1.nanoseconds_component());
    }

    #[test]
    fn test_string() {
        {
            let days_component = 3;
            let hours_component = 6;
            let minutes_component = 43;
            let seconds_component = 13;
            let nanoseconds_component = 123_456_789;
            let td1 = TimeDelta::new(
                days_component * NANOSECONDS_PER_DAY
                    + hours_component * NANOSECONDS_PER_HOUR
                    + minutes_component * NANOSECONDS_PER_MINUTE
                    + seconds_component * NANOSECONDS_PER_SECOND
                    + nanoseconds_component,
            );

            assert_eq!(
                td1.to_string(),
                format!(
                    "{} days {:02}:{:02}:{:02}.{:09}",
                    days_component,
                    hours_component,
                    minutes_component,
                    seconds_component,
                    nanoseconds_component
                )
            );
        }

        {
            let days_component = 0;
            let hours_component = 6;
            let minutes_component = 43;
            let seconds_component = 13;
            let nanoseconds_component = 123_456_789;
            let td1 = TimeDelta::new(
                days_component * NANOSECONDS_PER_DAY
                    + hours_component * NANOSECONDS_PER_HOUR
                    + minutes_component * NANOSECONDS_PER_MINUTE
                    + seconds_component * NANOSECONDS_PER_SECOND
                    + nanoseconds_component,
            );

            assert_eq!(
                td1.to_string(),
                format!(
                    "{:02}:{:02}:{:02}.{:09}",
                    hours_component, minutes_component, seconds_component, nanoseconds_component
                )
            );
        }

        {
            let days_component = 3;
            let hours_component = 6;
            let minutes_component = 43;
            let seconds_component = 13;
            let nanoseconds_component = 123_456_789;
            let td1 = TimeDelta::new(
                -(days_component * NANOSECONDS_PER_DAY
                    + hours_component * NANOSECONDS_PER_HOUR
                    + minutes_component * NANOSECONDS_PER_MINUTE
                    + seconds_component * NANOSECONDS_PER_SECOND
                    + nanoseconds_component),
            );

            assert_eq!(
                td1.to_string(),
                format!(
                    "-{} days {:02}:{:02}:{:02}.{:09}",
                    days_component,
                    hours_component,
                    minutes_component,
                    seconds_component,
                    nanoseconds_component
                )
            );
        }

        {
            let days_component = 0;
            let hours_component = 6;
            let minutes_component = 43;
            let seconds_component = 13;
            let nanoseconds_component = 123_456_789;
            let td1 = TimeDelta::new(
                -(days_component * NANOSECONDS_PER_DAY
                    + hours_component * NANOSECONDS_PER_HOUR
                    + minutes_component * NANOSECONDS_PER_MINUTE
                    + seconds_component * NANOSECONDS_PER_SECOND
                    + nanoseconds_component),
            );

            assert_eq!(
                td1.to_string(),
                format!(
                    "-{:02}:{:02}:{:02}.{:09}",
                    hours_component, minutes_component, seconds_component, nanoseconds_component
                )
            );
        }
    }
}
