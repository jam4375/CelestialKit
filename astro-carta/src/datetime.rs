pub struct DateTime {
    ticks: i128,
}

/// Returns the number of days in the given month considering leap years.
///
/// # Arguments
///
/// * `month_id` - An integer representing the month (1 for January, 2 for February, etc.).
/// * `is_leap_year` - A boolean indicating whether the year is a leap year or not.
///
/// # Returns
///
/// * `Some(days)` - The number of days in the specified month, considering if it's a leap year.
/// * `None` - If the `month_id` is not valid (outside the range 1-12).
///
/// # Examples
///
/// ```
/// use crate::astro_carta::datetime::days_in_month;
///
/// let january_days = days_in_month(1, false);
/// assert_eq!(january_days, Some(31));
///
/// let february_days_common = days_in_month(2, false);
/// assert_eq!(february_days_common, Some(28));
///
/// let february_days_leap = days_in_month(2, true);
/// assert_eq!(february_days_leap, Some(29));
///
/// let invalid_month = days_in_month(13, false);
/// assert_eq!(invalid_month, None);
/// ```
pub const fn days_in_month(month_id: u8, is_leap_year: bool) -> Option<u8> {
    let days_in_month = match month_id {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => return None,
    };

    if month_id == 2 && is_leap_year {
        Some(days_in_month + 1)
    } else {
        Some(days_in_month)
    }
}
