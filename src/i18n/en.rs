use std::ops::{Add, Sub};

use chrono::{Days, Local, NaiveDate, Weekday};
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    combinator::{map_res, value},
};

use crate::{
    i18n::naive_date_for_weekday,
    numeric::{dd_mm_only, dd_mm_y4, dd_only, mm_dd_only, mm_dd_y4},
    types::IResult,
};

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

/// Recognizes the `case insensitive` word `yesterday` in `Russian` and returns
/// the corresponding [`Option<NaiveDate>`] for it.
///
/// Is used to provide the handy way to combine it with
/// other parsers returning [`Option<NaiveDate>`], so it always `Some(_)`.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use nom::branch::alt;
/// use nom_date_parsers::{i18n::ru::yesterday_opt, numeric::dd_only};
///
/// fn versatile_date_parser(input: &str) -> Option<NaiveDate> {
///     alt((
///         dd_only,
///         // Here we can't use the plain "yesterday" parser
///         yesterday_opt,
///     ))(input)
///     .ok()
///     .and_then(|p| p.1)
/// }
/// ```
pub fn yesterday_opt(input: &str) -> IResult<&str, Option<NaiveDate>> {
    yesterday(input).map(|r| (r.0, Some(r.1)))
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

/// Recognizes the `case insensitive` word `tomorrow` in `English` and returns
/// the corresponding [`Option<NaiveDate>`] for it.
///
/// Is used to provide the handy way to combine it with
/// other parsers returning [`Option<NaiveDate>`], so it always `Some(_)`.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use nom::branch::alt;
/// use nom_date_parsers::{i18n::en::tomorrow_opt, numeric::dd_only};
///
/// fn versatile_date_parser(input: &str) -> Option<NaiveDate> {
///     alt((
///         dd_only,
///         // Here we can't use the plain "tomorrow" parser
///         tomorrow_opt,
///     ))(input)
///     .ok()
///     .and_then(|p| p.1)
/// }
/// ```
pub fn tomorrow_opt(input: &str) -> IResult<&str, Option<NaiveDate>> {
    tomorrow(input).map(|r| (r.0, Some(r.1)))
}

/// Recognizes the `case insensitive` short-named weekday in `English`.
///
/// The following words are accepted:
/// - `mon` -> [`Weekday::Mon`]
/// - `tue` | `tues` -> [`Weekday::Tue`]
/// - `wed` -> [`Weekday::Wed`]
/// - `thu` | `thur` | `thurs` -> [`Weekday::Thu`]
/// - `fri` -> [`Weekday::Fri`]
/// - `sat` -> [`Weekday::Sat`]
/// - `sun` -> [`Weekday::Sun`]
///
/// # Examples
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

/// Recognizes the `case insensitive` full-named weekday in `English`.
///
/// The following words are accepted:
/// - `monday` -> [`Weekday::Mon`]
/// - `tuesday` -> [`Weekday::Tue`]
/// - `wednesday` -> [`Weekday::Wed`]
/// - `thursday` -> [`Weekday::Thu`]
/// - `friday` -> [`Weekday::Fri`]
/// - `saturday` -> [`Weekday::Sat`]
/// - `sunday` -> [`Weekday::Sun`]
///
/// # Examples
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

/// Recognizes either the `case insensitive` short-named or full-named weekday
/// in `English`. Uses the [`short_named_weekday`] and [`full_named_weekday`]
/// parsers.
///
/// # Examples
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

/// Recognizes the `case insensitive` weekday in `English` using the
/// [`named_weekday`] function and returns the corresponding [`NaiveDate`]
/// for the current week.
///
/// # Examples
///
/// ```
/// use chrono::{NaiveDate, Weekday};
/// use nom_date_parsers::i18n::{en::current_named_weekday_only, naive_date_for_weekday};
///
/// assert_eq!(
///     current_named_weekday_only("Wednesday")?.1,
///     naive_date_for_weekday(Weekday::Wed)
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn current_named_weekday_only(input: &str) -> IResult<&str, NaiveDate> {
    map_res(named_weekday, |weekday: Weekday| {
        Ok(naive_date_for_weekday(weekday))
    })(input)
}

/// Recognizes the `case insensitive` weekday in `English` using the
/// [`current_named_weekday_only`] parser and returns the [`Option<NaiveDate>`].
///
/// Is used to provide the handy way to combine it with
/// other parsers returning [`Option<NaiveDate>`], so it always returns
/// `Some(_)`.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use nom::branch::alt;
/// use nom_date_parsers::{i18n::en::current_named_weekday_only_opt, numeric::dd_only};
///
/// fn versatile_date_parser(input: &str) -> Option<NaiveDate> {
///     alt((
///         dd_only,
///         // Here we can't use the plain "current_named_weekday_only" parser
///         current_named_weekday_only_opt,
///     ))(input)
///     .ok()
///     .and_then(|p| p.1)
/// }
/// ```
pub fn current_named_weekday_only_opt(input: &str) -> IResult<&str, Option<NaiveDate>> {
    current_named_weekday_only(input).map(|r| (r.0, Some(r.1)))
}

/// Uses the following parsers to recognize the `numeric` and
/// `language-specific` dates in `English`. Uses the `day-month-year` sequence:
/// - Numeric date parsers:
///     - [`dd_only`]
///     - [`dd_mm_only`]
///     - [`dd_mm_y4`]
/// - Language-specific
///     - [`yesterday_opt`]
///     - [`tomorrow_opt`]
///     - [`current_named_weekday_only_opt`]
pub fn bundle_dmy(input: &str) -> IResult<&str, Option<NaiveDate>> {
    alt((
        dd_mm_y4,
        dd_mm_only,
        dd_only,
        yesterday_opt,
        tomorrow_opt,
        current_named_weekday_only_opt,
    ))(input)
}

/// Uses the following parsers to recognize the `numeric` and
/// `language-specific` dates in `English`. Uses the `month-day-year` sequence:
/// - Numeric date parsers:
///     - [`dd_only`]
///     - [`mm_dd_only`]
///     - [`mm_dd_y4`]
/// - Language-specific
///     - [`yesterday_opt`]
///     - [`tomorrow_opt`]
///     - [`current_named_weekday_only_opt`]
pub fn bundle_mdy(input: &str) -> IResult<&str, Option<NaiveDate>> {
    alt((
        mm_dd_y4,
        mm_dd_only,
        dd_only,
        yesterday_opt,
        tomorrow_opt,
        current_named_weekday_only_opt,
    ))(input)
}

#[cfg(test)]
mod tests {
    use chrono::Datelike;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("Yesterday", Ok(("", Local::now().sub(Days::new(1)).date_naive())))]
    fn test_yesterday(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(yesterday(input), expected);
    }

    #[rstest]
    #[case("Yesterday", Ok(("", Some(Local::now().sub(Days::new(1)).date_naive()))))]
    fn test_yesterday_opt(#[case] input: &str, #[case] expected: IResult<&str, Option<NaiveDate>>) {
        assert_eq!(yesterday_opt(input), expected);
    }

    #[rstest]
    #[case("Tomorrow", Ok(("", Local::now().add(Days::new(1)).date_naive())))]
    fn test_tomorrow(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(tomorrow(input), expected);
    }

    #[rstest]
    #[case("Tomorrow", Ok(("", Some(Local::now().add(Days::new(1)).date_naive()))))]
    fn test_tomorrow_opt(#[case] input: &str, #[case] expected: IResult<&str, Option<NaiveDate>>) {
        assert_eq!(tomorrow_opt(input), expected);
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

    #[rstest]
    #[case("mon", Ok(("", Some(naive_date_for_weekday(Weekday::Mon)))))]
    #[case("Tuesday", Ok(("", Some(naive_date_for_weekday(Weekday::Tue)))))]
    fn test_current_named_weekday_only_opt(
        #[case] input: &str,
        #[case] expected: IResult<&str, Option<NaiveDate>>,
    ) {
        assert_eq!(current_named_weekday_only_opt(input), expected)
    }

    #[rstest]
    #[case("09", Ok(("", Local::now().date_naive().with_day(9))))]
    #[case("03/12", Ok(("", Local::now().date_naive().with_day(3).unwrap().with_month(12))))]
    #[case("13    06\t2024", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13))))]
    #[case("Yesterday", Ok(("", Some(Local::now().sub(Days::new(1)).date_naive()))))]
    #[case("Tomorrow", Ok(("", Some(Local::now().add(Days::new(1)).date_naive()))))]
    fn test_bundle_dmy(#[case] input: &str, #[case] expected: IResult<&str, Option<NaiveDate>>) {
        assert_eq!(bundle_dmy(input), expected)
    }

    #[rstest]
    #[case("09", Ok(("", Local::now().date_naive().with_day(9))))]
    #[case("12/03", Ok(("", Local::now().date_naive().with_day(3).unwrap().with_month(12))))]
    #[case("06    13\t2024", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13))))]
    #[case("Yesterday", Ok(("", Some(Local::now().sub(Days::new(1)).date_naive()))))]
    #[case("Tomorrow", Ok(("", Some(Local::now().add(Days::new(1)).date_naive()))))]
    fn test_bundle_mdy(#[case] input: &str, #[case] expected: IResult<&str, Option<NaiveDate>>) {
        assert_eq!(bundle_mdy(input), expected)
    }
}
