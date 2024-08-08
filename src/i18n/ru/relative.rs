use std::ops::{Add, Sub};

use chrono::{Days, Local, NaiveDate};
use nom::{bytes::complete::tag_no_case, combinator::value};

use crate::types::IResult;

/// Recognizes the `case insensitive` word `позавчера` in `Russian` and returns
/// the corresponding [`NaiveDate`].
///
/// # Examples
///
/// ```
/// use std::ops::Sub;
///
/// use chrono::{Days, Local, NaiveDate};
/// use nom_date_parsers::i18n::ru::day_before_yesterday;
///
/// assert_eq!(
///     day_before_yesterday("Позавчера")?.1,
///     Local::now().sub(Days::new(2)).date_naive()
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn day_before_yesterday(input: &str) -> IResult<&str, NaiveDate> {
    value(
        Local::now().sub(Days::new(2)).date_naive(),
        tag_no_case("позавчера"),
    )(input)
}

/// Recognizes the `case insensitive` word `вчера` in `Russian` and returns
/// the corresponding [`NaiveDate`].
///
/// # Examples
///
/// ```
/// use std::ops::Sub;
///
/// use chrono::{Days, Local, NaiveDate};
/// use nom_date_parsers::i18n::ru::yesterday;
///
/// assert_eq!(
///     yesterday("Вчера")?.1,
///     Local::now().sub(Days::new(1)).date_naive()
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn yesterday(input: &str) -> IResult<&str, NaiveDate> {
    value(
        Local::now().sub(Days::new(1)).date_naive(),
        tag_no_case("вчера"),
    )(input)
}

/// Recognizes the `case insensitive` word `today` in `Russian` and returns
/// the corresponding [`NaiveDate`] for it.
///
/// # Examples
///
/// ```
/// use chrono::Local;
/// use nom_date_parsers::i18n::ru::today;
///
/// assert_eq!(today("сегодня")?.1, Local::now().date_naive());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn today(input: &str) -> IResult<&str, NaiveDate> {
    value(Local::now().date_naive(), tag_no_case("сегодня"))(input)
}

/// Recognizes the `case insensitive` word `завтра` in `Russian` and returns the
/// corresponding [`NaiveDate`].
///
/// # Examples
///
/// ```
/// use std::ops::Add;
///
/// use chrono::{Days, Local, NaiveDate};
/// use nom_date_parsers::i18n::ru::tomorrow;
///
/// assert_eq!(
///     tomorrow("завтра")?.1,
///     Local::now().add(Days::new(1)).date_naive()
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn tomorrow(input: &str) -> IResult<&str, NaiveDate> {
    value(
        Local::now().add(Days::new(1)).date_naive(),
        tag_no_case("завтра"),
    )(input)
}

/// Recognizes the `case insensitive` word `послезавтра` in `Russian` and
/// returns the corresponding [`NaiveDate`].
///
/// # Examples
///
/// ```
/// use std::ops::Add;
///
/// use chrono::{Days, Local, NaiveDate};
/// use nom_date_parsers::i18n::ru::day_after_tomorrow;
///
/// assert_eq!(
///     day_after_tomorrow("послезавтра")?.1,
///     Local::now().add(Days::new(2)).date_naive()
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn day_after_tomorrow(input: &str) -> IResult<&str, NaiveDate> {
    value(
        Local::now().add(Days::new(2)).date_naive(),
        tag_no_case("послезавтра"),
    )(input)
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("позавчера", Ok(("", Local::now().sub(Days::new(2)).date_naive())))]
    fn test_day_before_yesterday(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(day_before_yesterday(input), expected);
    }

    #[rstest]
    #[case("Вчера", Ok(("", Local::now().sub(Days::new(1)).date_naive())))]
    fn test_yesterday(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(yesterday(input), expected);
    }

    #[rstest]
    #[case("Сегодня", Ok(("", Local::now().date_naive())))]
    fn test_today(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(today(input), expected);
    }

    #[rstest]
    #[case("Завтра", Ok(("", Local::now().add(Days::new(1)).date_naive())))]
    fn test_tomorrow(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(tomorrow(input), expected);
    }

    #[rstest]
    #[case("Послезавтра", Ok(("", Local::now().add(Days::new(2)).date_naive())))]
    fn test_day_after_tomorrow(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(day_after_tomorrow(input), expected);
    }
}
