use std::ops::{Add, Sub};

use chrono::{Days, Local, NaiveDate, Weekday};
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    combinator::{map_res, value},
};

use crate::{i18n::naive_date_for_weekday, types::IResult};

/// Recognizes the yesterday `case insensitive` word in `Russian` and returns the corresponding [`NaiveDate`] for it
///
/// ```
/// use std::ops::Sub;
///
/// use chrono::{Local, NaiveDate, Days};
/// use nom_date_parsers::i18n::ru::yesterday;
///
/// assert_eq!(yesterday("Вчера")?.1, Local::now().sub(Days::new(1)).date_naive());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn yesterday(input: &str) -> IResult<&str, NaiveDate> {
    value(
        Local::now().sub(Days::new(1)).date_naive(),
        tag_no_case("вчера"),
    )(input)
}

/// Recognizes the tomorrow `case insensitive` word in `Russian` and returns the corresponding [`NaiveDate`] for it
///
/// ```
/// use std::ops::Add;
///
/// use chrono::{Local, NaiveDate, Days};
/// use nom_date_parsers::i18n::ru::tomorrow;
///
/// assert_eq!(tomorrow("завтра")?.1, Local::now().add(Days::new(1)).date_naive());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn tomorrow(input: &str) -> IResult<&str, NaiveDate> {
    value(
        Local::now().add(Days::new(1)).date_naive(),
        tag_no_case("завтра"),
    )(input)
}

/// Recognizes the `case insensitive` short-named weekday in `Russian`
///
/// ```
/// use chrono::Weekday;
/// use nom_date_parsers::{
///     i18n::ru::short_named_weekday,
/// };
///
/// assert_eq!(short_named_weekday("пт")?.1, Weekday::Fri);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn short_named_weekday(input: &str) -> IResult<&str, Weekday> {
    alt((
        value(Weekday::Mon, tag_no_case("пн")),
        value(Weekday::Tue, tag_no_case("вт")),
        value(Weekday::Wed, tag_no_case("ср")),
        value(Weekday::Thu, tag_no_case("чт")),
        value(Weekday::Fri, tag_no_case("пт")),
        value(Weekday::Sat, tag_no_case("сб")),
        value(Weekday::Sun, tag_no_case("вс")),
    ))(input)
}

/// Recognizes the `case insensitive` full-named weekday in `Russian`
///
/// ```
/// use chrono::Weekday;
/// use nom_date_parsers::{
///     i18n::ru::full_named_weekday,
/// };
///
/// assert_eq!(full_named_weekday("среда")?.1, Weekday::Wed);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn full_named_weekday(input: &str) -> IResult<&str, Weekday> {
    alt((
        value(Weekday::Mon, tag_no_case("понедельник")),
        value(Weekday::Tue, tag_no_case("вторник")),
        value(Weekday::Wed, tag_no_case("среда")),
        value(Weekday::Thu, tag_no_case("четверг")),
        value(Weekday::Fri, tag_no_case("пятница")),
        value(Weekday::Sat, tag_no_case("суббота")),
        value(Weekday::Sun, tag_no_case("воскресенье")),
    ))(input)
}

/// Recognizes either the `case insensitive` short-named or full-named weekday in `Russian`
///
/// ```
/// use chrono::Weekday;
/// use nom_date_parsers::{
///     i18n::ru::named_weekday,
/// };
///
/// assert_eq!(named_weekday("пт")?.1, Weekday::Fri);
/// assert_eq!(named_weekday("Пятница")?.1, Weekday::Fri);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn named_weekday(input: &str) -> IResult<&str, Weekday> {
    alt((full_named_weekday, short_named_weekday))(input)
}

/// Recognizes the `case insensitive` weekday in `Russian` using the [`named_weekday`] function and returns the corresponding [`NaiveDate`]
/// for the current week
///
/// ```
/// use chrono::{NaiveDate, Weekday};
/// use nom_date_parsers::i18n::{
///     naive_date_for_weekday,
///     ru::current_named_weekday_only
/// };
///
/// assert_eq!(current_named_weekday_only("Среда")?.1, naive_date_for_weekday(Weekday::Wed));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn current_named_weekday_only(input: &str) -> IResult<&str, NaiveDate> {
    map_res(named_weekday, |weekday: Weekday| {
        Ok(naive_date_for_weekday(weekday))
    })(input)
}

// pub fn next_week_named_weekday_only(input: &str) -> IResult<&str, NaiveDate> {}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("Вчера", Ok(("", Local::now().sub(Days::new(1)).date_naive())))]
    fn test_yesterday(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(yesterday(input), expected);
    }

    #[rstest]
    #[case("Завтра", Ok(("", Local::now().add(Days::new(1)).date_naive())))]
    fn test_tomorrow(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(tomorrow(input), expected);
    }

    #[rstest]
    #[case("пн", Ok(("", Weekday::Mon)))]
    #[case("ВТ", Ok(("", Weekday::Tue)))]
    #[case("Ср", Ok(("", Weekday::Wed)))]
    fn test_short_named_weekday(#[case] input: &str, #[case] expected: IResult<&str, Weekday>) {
        assert_eq!(short_named_weekday(input), expected);
    }

    #[rstest]
    #[case("понедельник", Ok(("", Weekday::Mon)))]
    #[case("Вторник", Ok(("", Weekday::Tue)))]
    #[case("СРЕДА", Ok(("", Weekday::Wed)))]
    fn test_full_named_weekday(#[case] input: &str, #[case] expected: IResult<&str, Weekday>) {
        assert_eq!(full_named_weekday(input), expected);
    }

    #[rstest]
    #[case("пн", Ok(("", Weekday::Mon)))]
    #[case("вторник", Ok(("", Weekday::Tue)))]
    fn test_named_weekday(#[case] input: &str, #[case] expected: IResult<&str, Weekday>) {
        assert_eq!(named_weekday(input), expected)
    }

    #[rstest]
    #[case("пн", Ok(("", naive_date_for_weekday(Weekday::Mon))))]
    #[case("Вторник", Ok(("", naive_date_for_weekday(Weekday::Tue))))]
    fn test_current_named_weekday_only(
        #[case] input: &str,
        #[case] expected: IResult<&str, NaiveDate>,
    ) {
        assert_eq!(current_named_weekday_only(input), expected)
    }
}
