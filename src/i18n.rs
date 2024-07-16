use chrono::{Datelike, Local, NaiveDate, TimeDelta, Weekday};

#[cfg(feature = "en")]
pub mod en;
#[cfg(feature = "ru")]
pub mod ru;

/// Returns the [`NaiveDate`] for the specified [`Weekday`] in the current week
///
/// Suppose today is `16/07/2024`, so the `naive_date_for_weekday(Weekday::Mon)` will return the `15/07/2024`
/// and the `naive_date_for_weekday(Weekday::Sat)` will return the `21/07/2024`
pub fn naive_date_for_weekday(weekday: Weekday) -> NaiveDate {
    let now = Local::now().date_naive();
    now.checked_add_signed(TimeDelta::try_days(weekday as i64 - now.weekday() as i64).unwrap())
        .unwrap()
}
