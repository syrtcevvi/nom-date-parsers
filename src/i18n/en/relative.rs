use std::ops::{Add, Sub};

use chrono::{Days, Local, NaiveDate};
use nom::{bytes::complete::tag_no_case, combinator::value};

use crate::types::IResult;

/// Recognizes the `case insensitive` word `yesterday` in `English` and returns
/// the corresponding [`NaiveDate`] for it.
///
/// # Examples
///
/// ```
/// use std::ops::Sub;
///
/// use chrono::{Days, Local, NaiveDate};
/// use nom_date_parsers::i18n::en::yesterday;
///
/// assert_eq!(
///     yesterday("Yesterday")?.1,
///     Local::now().sub(Days::new(1)).date_naive()
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn yesterday(input: &str) -> IResult<&str, NaiveDate> {
    value(
        Local::now().sub(Days::new(1)).date_naive(),
        tag_no_case("yesterday"),
    )(input)
}

/// Recognizes the `case insensitive` word `tomorrow` in `English` and returns
/// the corresponding [`NaiveDate`] for it.
///
/// # Examples
///
/// ```
/// use std::ops::Add;
///
/// use chrono::{Days, Local, NaiveDate};
/// use nom_date_parsers::i18n::en::tomorrow;
///
/// assert_eq!(
///     tomorrow("tomorrow")?.1,
///     Local::now().add(Days::new(1)).date_naive()
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn tomorrow(input: &str) -> IResult<&str, NaiveDate> {
    value(
        Local::now().add(Days::new(1)).date_naive(),
        tag_no_case("tomorrow"),
    )(input)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("Yesterday", Ok(("", Local::now().sub(Days::new(1)).date_naive())))]
    fn test_yesterday(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(yesterday(input), expected);
    }

    #[rstest]
    #[case("Tomorrow", Ok(("", Local::now().add(Days::new(1)).date_naive())))]
    fn test_tomorrow(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(tomorrow(input), expected);
    }
}
