#![cfg(test)]

use crate::calendar::Calendar;
use crate::datetime::DateTime;

#[test]
pub fn test_from_date() {
    let cal = Calendar::default();
    let x = DateTime::from_date(&cal, 1011, 02, 22);
    assert_eq!(
        x.with_calendar(&cal).era().unwrap().name(),
        "Common Era".to_string()
    );
    assert_eq!(x.with_calendar(&cal).year(), 1011);
}
