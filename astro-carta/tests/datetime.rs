use astro_carta::datetime;

#[test]
fn days_in_month_test() {
    // January
    let days = datetime::days_in_month(1, false);
    assert_eq!(days, Some(31));
    let days = datetime::days_in_month(1, true);
    assert_eq!(days, Some(31));

    // February
    let days = datetime::days_in_month(2, false);
    assert_eq!(days, Some(28));
    let days = datetime::days_in_month(2, true);
    assert_eq!(days, Some(29));

    // March
    let days = datetime::days_in_month(3, false);
    assert_eq!(days, Some(31));
    let days = datetime::days_in_month(3, true);
    assert_eq!(days, Some(31));

    // April
    let days = datetime::days_in_month(4, false);
    assert_eq!(days, Some(30));
    let days = datetime::days_in_month(4, true);
    assert_eq!(days, Some(30));

    // May
    let days = datetime::days_in_month(5, false);
    assert_eq!(days, Some(31));
    let days = datetime::days_in_month(5, true);
    assert_eq!(days, Some(31));

    // June
    let days = datetime::days_in_month(6, false);
    assert_eq!(days, Some(30));
    let days = datetime::days_in_month(6, true);
    assert_eq!(days, Some(30));

    // July
    let days = datetime::days_in_month(7, false);
    assert_eq!(days, Some(31));
    let days = datetime::days_in_month(7, true);
    assert_eq!(days, Some(31));

    // August
    let days = datetime::days_in_month(8, false);
    assert_eq!(days, Some(31));
    let days = datetime::days_in_month(8, true);
    assert_eq!(days, Some(31));

    // September
    let days = datetime::days_in_month(9, false);
    assert_eq!(days, Some(30));
    let days = datetime::days_in_month(9, true);
    assert_eq!(days, Some(30));

    // October
    let days = datetime::days_in_month(10, false);
    assert_eq!(days, Some(31));
    let days = datetime::days_in_month(10, true);
    assert_eq!(days, Some(31));

    // November
    let days = datetime::days_in_month(11, false);
    assert_eq!(days, Some(30));
    let days = datetime::days_in_month(11, true);
    assert_eq!(days, Some(30));

    // December
    let days = datetime::days_in_month(12, false);
    assert_eq!(days, Some(31));
    let days = datetime::days_in_month(12, true);
    assert_eq!(days, Some(31));

    // Error cases
    assert_eq!(datetime::days_in_month(13, false), None);
    assert_eq!(datetime::days_in_month(13, true), None);
    assert_eq!(datetime::days_in_month(0, false), None);
    assert_eq!(datetime::days_in_month(0, true), None);
}
