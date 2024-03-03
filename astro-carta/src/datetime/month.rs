/// Returns the number of days in the given month considering leap years.
///
/// # Arguments
///
/// * `month` - An integer representing the month (1 for January, 2 for February, etc.).
/// * `is_leap_year` - A boolean indicating whether the year is a leap year or not.
///
/// # Returns
///
/// * `Some(days)` - The number of days in the specified month, considering if it's a leap year.
/// * `None` - If the `month` is not valid (outside the range 1-12).
///
/// # Examples
///
/// ```ignore
/// use crate::astro_carta::datetime::month;
///
/// let january_days = month::days_in_month(1, false);
/// assert_eq!(january_days, Some(31));
///
/// let february_days_common = month::days_in_month(2, false);
/// assert_eq!(february_days_common, Some(28));
///
/// let february_days_leap = month::days_in_month(2, true);
/// assert_eq!(february_days_leap, Some(29));
///
/// let invalid_month = month::days_in_month(13, false);
/// assert_eq!(invalid_month, None);
/// ```
pub const fn days_in_month(month: u8, is_leap_year: bool) -> Option<u8> {
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => return None,
    };

    if month == 2 && is_leap_year {
        Some(days_in_month + 1)
    } else {
        Some(days_in_month)
    }
}

/// Calculates the cumulative days for the specified month in a given year.
///
/// # Arguments
///
/// * `month` - The month for which cumulative days are calculated. Should be between 1 and 12 (inclusive).
/// * `is_leap_year` - A boolean indicating whether the year is a leap year.
///
/// # Returns
///
/// * `Some(u16)` - The cumulative days up to the specified month in the given year, if the month is valid.
/// * `None` - If the provided month is not within the valid range (1-12).
///
/// # Examples
///
/// ```ignore
/// use crate::astro_carta::datetime::month;
///
/// let cumulative_days = month::cummulative_days_for_month(3, false);
/// assert_eq!(cumulative_days, Some(59));
///
/// let cumulative_days_leap_year = month::cummulative_days_for_month(3, true);
/// assert_eq!(cumulative_days_leap_year, Some(60));
///
/// let invalid_month = month::cummulative_days_for_month(13, false);
/// assert_eq!(invalid_month, None);
/// ```
pub const fn cummulative_days_for_month(month: u8, is_leap_year: bool) -> Option<u16> {
    if month < 1 || month > 12 {
        return None;
    }

    if is_leap_year {
        let cummulative_days_all: [u16; 12] =
            [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
        Some(cummulative_days_all[month as usize - 1])
    } else {
        let cummulative_days_all: [u16; 12] =
            [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        Some(cummulative_days_all[month as usize - 1])
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

    #[test]
    fn cummulative_days_for_month_test() {
        // Check against manual calculation:
        for is_leap_year in [false, true] {
            for month in 1..=12 {
                let mut cumul_days: u16 = 0;
                for ii in 1..month {
                    cumul_days += days_in_month(ii, is_leap_year).unwrap() as u16;
                }
                assert_eq!(
                    cummulative_days_for_month(month, is_leap_year),
                    Some(cumul_days)
                );
            }
        }

        // Error cases:
        assert_eq!(cummulative_days_for_month(13, false), None);
        assert_eq!(cummulative_days_for_month(13, true), None);
        assert_eq!(cummulative_days_for_month(0, false), None);
        assert_eq!(cummulative_days_for_month(0, true), None);
    }
}
