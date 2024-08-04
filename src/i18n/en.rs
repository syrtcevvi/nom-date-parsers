mod relative;
mod weekday;

use chrono::NaiveDate;
use nom::branch::alt;

use crate::{
    numeric::{dd_mm_only, dd_mm_y4, dd_only, mm_dd_only, mm_dd_y4},
    types::IResult,
};

pub use self::{relative::*, weekday::*};

/// Uses the following parsers to recognize the `numeric` and
/// `language-specific` dates in `English`. Uses the `day-month-year` sequence:
/// - Numeric date parsers:
///     - [`dd_mm_y4`]
///     - [`dd_mm_only`]
///     - [`dd_only`]
/// - Language-specific
///     - [`yesterday`]
///     - [`tomorrow`]
///     - [`current_named_weekday_only`]
///
/// If the specified date doesn't exist, returns `nom::Err::Error`
pub fn bundle_dmy(input: &str) -> IResult<&str, NaiveDate> {
    alt((
        dd_mm_y4,
        dd_mm_only,
        dd_only,
        yesterday,
        tomorrow,
        current_named_weekday_only,
    ))(input)
}

/// Uses the following parsers to recognize the `numeric` and
/// `language-specific` dates in `English`. Uses the `month-day-year` sequence:
/// - Numeric date parsers:
///     - [`mm_dd_y4`]
///     - [`mm_dd_only`]
///     - [`dd_only`]
/// - Language-specific
///     - [`yesterday`]
///     - [`tomorrow`]
///     - [`current_named_weekday_only`]
///
/// If the specified date doesn't exist, returns `nom::Err::Error`
pub fn bundle_mdy(input: &str) -> IResult<&str, NaiveDate> {
    alt((
        mm_dd_y4,
        mm_dd_only,
        dd_only,
        yesterday,
        tomorrow,
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
    #[case("09", Ok(("", Local::now().date_naive().with_day(9).unwrap())))]
    #[case("03/12", Ok(("", Local::now().date_naive().with_day(3).unwrap().with_month(12).unwrap())))]
    #[case("13    06\t2024", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13).unwrap())))]
    #[case("Yesterday", Ok(("", Local::now().sub(Days::new(1)).date_naive())))]
    #[case("Tomorrow", Ok(("", Local::now().add(Days::new(1)).date_naive())))]
    fn test_bundle_dmy(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(bundle_dmy(input), expected)
    }

    #[rstest]
    #[case("09", Ok(("", Local::now().date_naive().with_day(9).unwrap())))]
    #[case("12/03", Ok(("", Local::now().date_naive().with_day(3).unwrap().with_month(12).unwrap())))]
    #[case("06    13\t2024", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13).unwrap())))]
    #[case("Yesterday", Ok(("", Local::now().sub(Days::new(1)).date_naive())))]
    #[case("Tomorrow", Ok(("", Local::now().add(Days::new(1)).date_naive())))]
    fn test_bundle_mdy(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(bundle_mdy(input), expected)
    }
}
