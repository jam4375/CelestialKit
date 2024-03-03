pub struct DateTime {
    ticks: i128,
}

/// Calculates the number of days in a given month for a given year, considering leap years.
///
/// # Arguments
///
/// * `month_id` - An integer representing the month (1 for January, 2 for February, etc.).
/// * `is_leap_year` - A boolean indicating whether the year is a leap year or not.
///
/// # Returns
///
/// * `Result<i32, String>` - If `month_id` is valid, returns `Ok(i32)` with the number of days in the month.
///   If `month_id` is invalid, returns `Err(String)` with an error message.
///
/// # Examples
///
/// ```
/// use crate::astro_carta::datetime::days_in_month;
///
/// // February in a non-leap year
/// assert_eq!(days_in_month(2, false), Ok(28));
///
/// // February in a leap year
/// assert_eq!(days_in_month(2, true), Ok(29));
///
/// // April in a non-leap year
/// assert_eq!(days_in_month(4, false), Ok(30));
///
/// // Invalid month_id
/// assert_eq!(days_in_month(13, false), Err("Invalid month_id=13".to_string()));
/// ```
pub fn days_in_month(month_id: i32, is_leap_year: bool) -> Result<i32, String> {
    let days_in_month = match month_id {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => return Err(format!("Invalid month_id={:?}", month_id)),
    };

    if month_id == 2 && is_leap_year {
        Ok(days_in_month + 1)
    } else {
        Ok(days_in_month)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_in_month_test() {
        // January
        let days = days_in_month(1, false);
        assert_eq!(days, Ok(31));
        let days = days_in_month(1, true);
        assert_eq!(days, Ok(31));

        // February
        let days = days_in_month(2, false);
        assert_eq!(days, Ok(28));
        let days = days_in_month(2, true);
        assert_eq!(days, Ok(29));

        // March
        let days = days_in_month(3, false);
        assert_eq!(days, Ok(31));
        let days = days_in_month(3, true);
        assert_eq!(days, Ok(31));

        // April
        let days = days_in_month(4, false);
        assert_eq!(days, Ok(30));
        let days = days_in_month(4, true);
        assert_eq!(days, Ok(30));

        // May
        let days = days_in_month(5, false);
        assert_eq!(days, Ok(31));
        let days = days_in_month(5, true);
        assert_eq!(days, Ok(31));

        // June
        let days = days_in_month(6, false);
        assert_eq!(days, Ok(30));
        let days = days_in_month(6, true);
        assert_eq!(days, Ok(30));

        // July
        let days = days_in_month(7, false);
        assert_eq!(days, Ok(31));
        let days = days_in_month(7, true);
        assert_eq!(days, Ok(31));

        // August
        let days = days_in_month(8, false);
        assert_eq!(days, Ok(31));
        let days = days_in_month(8, true);
        assert_eq!(days, Ok(31));

        // September
        let days = days_in_month(9, false);
        assert_eq!(days, Ok(30));
        let days = days_in_month(9, true);
        assert_eq!(days, Ok(30));

        // October
        let days = days_in_month(10, false);
        assert_eq!(days, Ok(31));
        let days = days_in_month(10, true);
        assert_eq!(days, Ok(31));

        // November
        let days = days_in_month(11, false);
        assert_eq!(days, Ok(30));
        let days = days_in_month(11, true);
        assert_eq!(days, Ok(30));

        // December
        let days = days_in_month(12, false);
        assert_eq!(days, Ok(31));
        let days = days_in_month(12, true);
        assert_eq!(days, Ok(31));

        // Error cases
        assert_eq!(
            days_in_month(13, false),
            Err("Invalid month_id=13".to_string())
        );
        assert_eq!(
            days_in_month(13, true),
            Err("Invalid month_id=13".to_string())
        );
        assert_eq!(
            days_in_month(0, false),
            Err("Invalid month_id=0".to_string())
        );
        assert_eq!(
            days_in_month(0, true),
            Err("Invalid month_id=0".to_string())
        );
    }
}
