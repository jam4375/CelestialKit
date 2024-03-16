/// Determines whether a given year is a leap year in the proleptic Gregorian calendar.
///
/// # Arguments
///
/// * `year` - The year to be checked for leap year status.
///
/// # Returns
///
/// Returns `true` if the given year is a leap year, otherwise `false`.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(is_leap_year(2020), true);
/// assert_eq!(is_leap_year(1900), false);
/// assert_eq!(is_leap_year(2000), true);
/// assert_eq!(is_leap_year(2024), true);
/// assert_eq!(is_leap_year(2021), false);
/// ```
pub fn is_leap_year(year: i128) -> bool {
    (((year % 4) == 0) && (year % 100) != 0) || ((year % 400) == 0)
}

/// Determines if a given year is a valid Gregorian year.
///
/// A valid Gregorian year should be greater than or equal to 1 and less than or equal to 100,000,000,000.
/// The upper limit is an arbitrary choice that is extremely far into the future and still within the integer precision used.
///
/// # Arguments
///
/// * `year` - An integer representing the year to be checked for validity.
///
/// # Returns
///
/// Returns `true` if the year is valid, `false` otherwise.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(is_valid_year(2024), true);
/// assert_eq!(is_valid_year(0), false);
/// assert_eq!(is_valid_year(101_000_000_000), false);
/// ```
pub fn is_valid_year(year: i128) -> bool {
    if year < 1 || year > 100_000_000_000 {
        return false;
    }

    true
}

/// Checks if the given year and month values form a valid year-month combination in the proleptic Gregorian calendar.
///
/// # Arguments
///
/// * `year` - An integer representing the year.
/// * `month` - An integer representing the month (1-12).
///
/// # Returns
///
/// `true` if the year and month values are valid, `false` otherwise.
///
/// # Examples
///
/// ```ignore
/// assert!(is_valid_year_month(2024, 3));
/// assert!(!is_valid_year_month(2024, 13));
/// assert!(!is_valid_year_month(0, 3));
/// ```
pub fn is_valid_year_month(year: i128, month: i128) -> bool {
    if !is_valid_year(year) {
        return false;
    }

    if month < 1 || month > 12 {
        return false;
    }

    true
}

/// Checks if the provided year, month, and day form a valid date in the proleptic Gregorian calendar.
///
/// # Arguments
///
/// * `year` - An integer representing the year.
/// * `month` - An integer representing the month (1-based, January is 1).
/// * `day` - An integer representing the day of the month.
///
/// # Returns
///
/// A boolean value indicating whether the provided year, month, and day form a valid date (`true`)
/// or not (`false`).
///
/// # Examples
///
/// ```ignore
/// assert_eq!(is_valid_year_month_day(2024, 3, 16), true);
/// assert_eq!(is_valid_year_month_day(2024, 2, 29), true); // Leap year
/// assert_eq!(is_valid_year_month_day(2024, 2, 30), false); // Not a valid day in February
/// assert_eq!(is_valid_year_month_day(2024, 4, 31), false); // Not a valid day in April
/// assert_eq!(is_valid_year_month_day(2024, 13, 1), false); // Invalid month
/// ```
pub fn is_valid_year_month_day(year: i128, month: i128, day: i128) -> bool {
    if !is_valid_year_month(year, month) {
        return false;
    }

    let is_leap_year = is_leap_year(year);
    if day < 1 {
        return false;
    }

    if let Some(days_in_month) = super::month::days_in_month(month as u8, is_leap_year) {
        if day > days_in_month as i128 {
            return false;
        }
    } else {
        return false;
    }

    return true;
}

/// Calculates the day of the year from the given year, month, and day in the proleptic Gregorian calendar.
/// Returns `Some(day_of_year)` if the input is valid, otherwise returns `None`.
///
/// # Arguments
///
/// * `year` - The year (e.g., 2024).
/// * `month` - The month as an integer (1 for January, 2 for February, etc.).
/// * `day` - The day of the month.
///
/// # Returns
///
/// * `Some(day_of_year)` - The day of the year if the input is valid.
/// * `None` - If the input is not a valid date.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(day_of_year(2024, 3, 16), Some(76));
/// assert_eq!(day_of_year(2024, 2, 29), Some(60));
/// assert_eq!(day_of_year(2024, 13, 1), None);
/// ```
pub fn day_of_year(year: i128, month: i128, day: i128) -> Option<i128> {
    if !is_valid_year_month_day(year, month, day) {
        return None;
    }

    let is_leap_year = is_leap_year(year);
    if let Some(cumul_days) = super::month::cummulative_days_for_month(month as u8, is_leap_year) {
        Some(day + cumul_days as i128)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_year_test() {
        assert_eq!(is_leap_year(2020), true);
        assert_eq!(is_leap_year(1900), false);
        assert_eq!(is_leap_year(2000), true);
        assert_eq!(is_leap_year(2024), true);
        assert_eq!(is_leap_year(2021), false);
    }

    #[test]
    fn is_valid_year_test() {
        assert_eq!(is_valid_year(2024), true);
        assert_eq!(is_valid_year(0), false);
        assert_eq!(is_valid_year(100_000_000_001), false);
    }

    #[test]
    fn is_valid_year_month_test() {
        assert_eq!(is_valid_year_month(2024, 1), true);
        assert_eq!(is_valid_year_month(2024, 12), true);
        assert_eq!(is_valid_year_month(2024, 0), false);
        assert_eq!(is_valid_year_month(2024, 13), false);
        assert_eq!(is_valid_year_month(0, 12), false);
        assert_eq!(is_valid_year_month(100_000_000_001, 12), false);
    }

    #[test]
    fn is_valid_year_month_day_test() {
        assert_eq!(is_valid_year_month_day(2024, 3, 1), true);
        assert_eq!(is_valid_year_month_day(2024, 2, 29), true);
        assert_eq!(is_valid_year_month_day(2023, 2, 29), false);
        assert_eq!(is_valid_year_month_day(2023, 2, 28), true);
        assert_eq!(is_valid_year_month_day(2024, 3, 31), true);
        assert_eq!(is_valid_year_month_day(2024, 3, 32), false);
        assert_eq!(is_valid_year_month_day(2024, 3, 0), false);
        assert_eq!(is_valid_year_month_day(2024, 13, 1), false);
        assert_eq!(is_valid_year_month_day(2024, -1, 1), false);
        assert_eq!(is_valid_year_month_day(0, 3, 1), false);
        assert_eq!(is_valid_year_month_day(100_000_000_001, 3, 1), false);
    }

    #[test]
    fn day_of_year_test() {
        assert_eq!(day_of_year(2024, 3, 16), Some(76));
        assert_eq!(day_of_year(2023, 2, 29), None);
        assert_eq!(day_of_year(2024, 2, 29), Some(60));
        assert_eq!(day_of_year(2023, 12, 31), Some(365));
        assert_eq!(day_of_year(2024, 12, 31), Some(366));
        assert_eq!(day_of_year(2024, 13, 1), None);
        assert_eq!(day_of_year(2024, 1, 32), None);
        assert_eq!(day_of_year(-1, 3, 13), None);
    }
}
