use std::ops::{Add, Sub};

use chrono::{Days, Local, NaiveDate, Weekday};
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    combinator::{map_res, value},
};

use crate::{i18n::naive_date_for_weekday, types::IResult};

/// Recognizes the yesterday `case insensitive` word in `English` and returns the corresponding [`NaiveDate`] for it
///
/// ```
/// use std::ops::Sub;
///
/// use chrono::{Local, NaiveDate, Days};
/// use nom_date_parsers::i18n::en::yesterday;
///
/// assert_eq!(yesterday("Yesterday")?.1, Local::now().sub(Days::new(1)).date_naive());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn yesterday(input: &str) -> IResult<&str, NaiveDate> {
    value(
        Local::now().sub(Days::new(1)).date_naive(),
        tag_no_case("yesterday"),
    )(input)
}

/// Recognizes the tomorrow `case insensitive` word in `English` and returns the corresponding [`NaiveDate`] for it
///
/// ```
/// use std::ops::Add;
///
/// use chrono::{Local, NaiveDate, Days};
/// use nom_date_parsers::i18n::en::tomorrow;
///
/// assert_eq!(tomorrow("tomorrow")?.1, Local::now().add(Days::new(1)).date_naive());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn tomorrow(input: &str) -> IResult<&str, NaiveDate> {
    value(
        Local::now().add(Days::new(1)).date_naive(),
        tag_no_case("tomorrow"),
    )(input)
}

/// Recognizes the `case insensitive` short-named weekday in `English`
///
/// ```
/// use chrono::Weekday;
/// use nom_date_parsers::i18n::en::short_named_weekday;
///
/// assert_eq!(short_named_weekday("Sun")?.1, Weekday::Sun);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn short_named_weekday(input: &str) -> IResult<&str, Weekday> {
    alt((
        value(Weekday::Mon, tag_no_case("mon")),
        value(Weekday::Tue, tag_no_case("tue")),
        value(Weekday::Tue, tag_no_case("tues")),
        value(Weekday::Wed, tag_no_case("wed")),
        value(Weekday::Thu, tag_no_case("thu")),
        value(Weekday::Thu, tag_no_case("thur")),
        value(Weekday::Thu, tag_no_case("thurs")),
        value(Weekday::Fri, tag_no_case("fri")),
        value(Weekday::Sat, tag_no_case("sat")),
        value(Weekday::Sun, tag_no_case("sun")),
    ))(input)
}

/// Recognizes the `case insensitive` full-named weekday in `English`
///
/// ```
/// use chrono::Weekday;
/// use nom_date_parsers::i18n::en::full_named_weekday;
///
/// assert_eq!(full_named_weekday("Wednesday")?.1, Weekday::Wed);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn full_named_weekday(input: &str) -> IResult<&str, Weekday> {
    alt((
        value(Weekday::Mon, tag_no_case("monday")),
        value(Weekday::Tue, tag_no_case("tuesday")),
        value(Weekday::Wed, tag_no_case("wednesday")),
        value(Weekday::Thu, tag_no_case("thursday")),
        value(Weekday::Fri, tag_no_case("friday")),
        value(Weekday::Sat, tag_no_case("saturday")),
        value(Weekday::Sun, tag_no_case("sunday")),
    ))(input)
}

/// Recognizes either the `case insensitive` short-named or full-named weekday in `English`
///
/// ```
/// use chrono::Weekday;
/// use nom_date_parsers::i18n::en::named_weekday;
///
/// assert_eq!(named_weekday("Fri")?.1, Weekday::Fri);
/// assert_eq!(named_weekday("friday")?.1, Weekday::Fri);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn named_weekday(input: &str) -> IResult<&str, Weekday> {
    alt((full_named_weekday, short_named_weekday))(input)
}

/// Recognizes the `case insensitive` weekday in `English` using the [`named_weekday`] function and returns the corresponding [`NaiveDate`]
/// for the current week
///
/// ```
/// use chrono::{NaiveDate, Weekday};
/// use nom_date_parsers::i18n::{
///     naive_date_for_weekday,
///     en::current_named_weekday_only
/// };
///
/// assert_eq!(current_named_weekday_only("Wednesday")?.1, naive_date_for_weekday(Weekday::Wed));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn current_named_weekday_only(input: &str) -> IResult<&str, NaiveDate> {
    map_res(named_weekday, |weekday: Weekday| {
        Ok(naive_date_for_weekday(weekday))
    })(input)
}

#[cfg(test)]
mod tests {
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

    #[rstest]
    #[case("mon", Ok(("", Weekday::Mon)))]
    #[case("tue", Ok(("", Weekday::Tue)))]
    #[case("Wed", Ok(("", Weekday::Wed)))]
    fn test_short_named_weekday(#[case] input: &str, #[case] expected: IResult<&str, Weekday>) {
        assert_eq!(short_named_weekday(input), expected);
    }

    #[rstest]
    #[case("monday", Ok(("", Weekday::Mon)))]
    #[case("Tuesday", Ok(("", Weekday::Tue)))]
    #[case("Wednesday", Ok(("", Weekday::Wed)))]
    fn test_full_named_weekday(#[case] input: &str, #[case] expected: IResult<&str, Weekday>) {
        assert_eq!(full_named_weekday(input), expected);
    }

    #[rstest]
    #[case("mon", Ok(("", Weekday::Mon)))]
    #[case("Tuesday", Ok(("", Weekday::Tue)))]
    fn test_named_weekday(#[case] input: &str, #[case] expected: IResult<&str, Weekday>) {
        assert_eq!(named_weekday(input), expected)
    }

    #[rstest]
    #[case("mon", Ok(("", naive_date_for_weekday(Weekday::Mon))))]
    #[case("Tuesday", Ok(("", naive_date_for_weekday(Weekday::Tue))))]
    fn test_current_named_weekday_only(
        #[case] input: &str,
        #[case] expected: IResult<&str, NaiveDate>,
    ) {
        assert_eq!(current_named_weekday_only(input), expected)
    }
}
