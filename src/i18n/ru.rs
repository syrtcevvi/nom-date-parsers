mod relative;
mod weekday;

use chrono::NaiveDate;
use nom::branch::alt;

use crate::{
    numeric::{dd_mm_only, dd_mm_y4, dd_only},
    types::IResult,
};

pub use self::{relative::*, weekday::*};

/// Uses the following parsers to recognize the `numeric` and
/// `language-specific` dates in `Russian`:
/// - Numeric date parsers:
///     - [`dd_mm_y4`]
///     - [`dd_mm_only`]
///     - [`dd_only`]
/// - Language-specific
///     - [`day_before_yesterday`]
///     - [`yesterday`]
///     - [`tomorrow`]
///     - [`day_after_tomorrow`]
///     - [`current_named_weekday_only`]
///
/// If the specified date doesn't exist, returns `nom::Err::Error`
pub fn bundle(input: &str) -> IResult<&str, NaiveDate> {
    alt((
        dd_mm_y4,
        dd_mm_only,
        dd_only,
        day_before_yesterday,
        yesterday,
        tomorrow,
        day_after_tomorrow,
        current_named_weekday_only,
    ))(input)
}

#[cfg(test)]
mod tests {
    use std::ops::{Add, Sub};

    use chrono::{Datelike, Days, Local};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("1", Ok(("", Local::now().date_naive().with_day(1).unwrap())))]
    #[case("09", Ok(("", Local::now().date_naive().with_day(9).unwrap())))]
    #[case("03/12", Ok(("", NaiveDate::from_ymd_opt(Local::now().year(), 12, 3).unwrap())))]
    #[case("13    06\t2024", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13).unwrap())))]
    #[case("позавчера", Ok(("", Local::now().sub(Days::new(2)).date_naive())))]
    #[case("Вчера", Ok(("", Local::now().sub(Days::new(1)).date_naive())))]
    #[case("Завтра", Ok(("", Local::now().add(Days::new(1)).date_naive())))]
    #[case("послезавтра", Ok(("", Local::now().add(Days::new(2)).date_naive())))]
    fn test_bundle(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(bundle(input), expected)
    }
}
