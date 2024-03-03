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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_in_month_test() {
        // January
        let days = days_in_month(1, false);
        assert_eq!(days, Some(31));
        let days = days_in_month(1, true);
        assert_eq!(days, Some(31));

        // February
        let days = days_in_month(2, false);
        assert_eq!(days, Some(28));
        let days = days_in_month(2, true);
        assert_eq!(days, Some(29));

        // March
        let days = days_in_month(3, false);
        assert_eq!(days, Some(31));
        let days = days_in_month(3, true);
        assert_eq!(days, Some(31));

        // April
        let days = days_in_month(4, false);
        assert_eq!(days, Some(30));
        let days = days_in_month(4, true);
        assert_eq!(days, Some(30));

        // May
        let days = days_in_month(5, false);
        assert_eq!(days, Some(31));
        let days = days_in_month(5, true);
        assert_eq!(days, Some(31));

        // June
        let days = days_in_month(6, false);
        assert_eq!(days, Some(30));
        let days = days_in_month(6, true);
        assert_eq!(days, Some(30));

        // July
        let days = days_in_month(7, false);
        assert_eq!(days, Some(31));
        let days = days_in_month(7, true);
        assert_eq!(days, Some(31));

        // August
        let days = days_in_month(8, false);
        assert_eq!(days, Some(31));
        let days = days_in_month(8, true);
        assert_eq!(days, Some(31));

        // September
        let days = days_in_month(9, false);
        assert_eq!(days, Some(30));
        let days = days_in_month(9, true);
        assert_eq!(days, Some(30));

        // October
        let days = days_in_month(10, false);
        assert_eq!(days, Some(31));
        let days = days_in_month(10, true);
        assert_eq!(days, Some(31));

        // November
        let days = days_in_month(11, false);
        assert_eq!(days, Some(30));
        let days = days_in_month(11, true);
        assert_eq!(days, Some(30));

        // December
        let days = days_in_month(12, false);
        assert_eq!(days, Some(31));
        let days = days_in_month(12, true);
        assert_eq!(days, Some(31));

        // Error cases
        assert_eq!(days_in_month(13, false), None);
        assert_eq!(days_in_month(13, true), None);
        assert_eq!(days_in_month(0, false), None);
        assert_eq!(days_in_month(0, true), None);
    }
}
