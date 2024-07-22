use std::ops::{Add, Sub};

use chrono::{Days, Local, NaiveDate, Weekday};
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    combinator::{map_res, value},
};

use crate::{
    i18n::naive_date_for_weekday,
    numeric::{dd_mm_only, dd_mm_y4, dd_only},
    types::IResult,
};

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

/// Recognizes the `case insensitive` word `позавчера` in `Russian` and returns
/// the corresponding [`NaiveDate`].
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
/// use nom_date_parsers::{i18n::ru::day_before_yesterday_opt, numeric::dd_only};
///
/// fn versatile_date_parser(input: &str) -> Option<NaiveDate> {
///     alt((
///         dd_only,
///         // Here we can't use the plain "day_before_yesterday" parser
///         day_before_yesterday_opt,
///     ))(input)
///     .ok()
///     .and_then(|p| p.1)
/// }
/// ```
pub fn day_before_yesterday_opt(input: &str) -> IResult<&str, Option<NaiveDate>> {
    day_before_yesterday(input).map(|r| (r.0, Some(r.1)))
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

/// Recognizes the `case insensitive` word `вчера` in `Russian` and returns the
/// corresponding [`Option<NaiveDate>`].
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

/// Recognizes the `case insensitive` word `завтра` in `Russian` and returns the
/// corresponding [`Option<NaiveDate>`].
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
/// use nom_date_parsers::{i18n::ru::tomorrow_opt, numeric::dd_only};
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

/// Recognizes the `case insensitive` word `послезавтра` in `Russian` and
/// returns the corresponding [`Option<NaiveDate>`].
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
/// use nom_date_parsers::{i18n::ru::day_after_tomorrow_opt, numeric::dd_only};
///
/// fn versatile_date_parser(input: &str) -> Option<NaiveDate> {
///     alt((
///         dd_only,
///         // Here we can't use the plain "day_after_tomorrow" parser
///         day_after_tomorrow_opt,
///     ))(input)
///     .ok()
///     .and_then(|p| p.1)
/// }
/// ```
pub fn day_after_tomorrow_opt(input: &str) -> IResult<&str, Option<NaiveDate>> {
    day_after_tomorrow(input).map(|r| (r.0, Some(r.1)))
}

/// Recognizes the `case insensitive` short-named weekday in `Russian`.
///
/// The following words are accepted:
/// - `пн` -> [`Weekday::Mon`]
/// - `вт` -> [`Weekday::Tue`]
/// - `ср` -> [`Weekday::Wed`]
/// - `чт` -> [`Weekday::Thu`]
/// - `пт` -> [`Weekday::Fri`]
/// - `сб` -> [`Weekday::Sat`]
/// - `вс` -> [`Weekday::Sun`]
///
/// # Examples
///
/// ```
/// use chrono::Weekday;
/// use nom_date_parsers::i18n::ru::short_named_weekday;
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

/// Recognizes the `case insensitive` full-named weekday in `Russian`.
///
/// The following words are accepted:
/// - `понедельник` -> [`Weekday::Mon`]
/// - `вторник` -> [`Weekday::Tue`]
/// - `среда` -> [`Weekday::Wed`]
/// - `четверг` -> [`Weekday::Thu`]
/// - `пятница` -> [`Weekday::Fri`]
/// - `суббота` -> [`Weekday::Sat`]
/// - `воскресенье` -> [`Weekday::Sun`]
///
/// # Examples
///
/// ```
/// use chrono::Weekday;
/// use nom_date_parsers::i18n::ru::full_named_weekday;
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

/// Recognizes either the `case insensitive` short-named or full-named weekday
/// in `Russian`. Uses the [`short_named_weekday`] and [`full_named_weekday`]
/// parsers.
///
/// # Examples
///
/// ```
/// use chrono::Weekday;
/// use nom_date_parsers::i18n::ru::named_weekday;
///
/// assert_eq!(named_weekday("пт")?.1, Weekday::Fri);
/// assert_eq!(named_weekday("Пятница")?.1, Weekday::Fri);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn named_weekday(input: &str) -> IResult<&str, Weekday> {
    alt((full_named_weekday, short_named_weekday))(input)
}

/// Recognizes the `case insensitive` weekday in `Russian` using the
/// [`named_weekday`] parser and returns the corresponding [`NaiveDate`]
/// for the current week.
///
/// # Examples
///
/// ```
/// use chrono::{NaiveDate, Weekday};
/// use nom_date_parsers::i18n::{naive_date_for_weekday, ru::current_named_weekday_only};
///
/// assert_eq!(
///     current_named_weekday_only("Среда")?.1,
///     naive_date_for_weekday(Weekday::Wed)
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn current_named_weekday_only(input: &str) -> IResult<&str, NaiveDate> {
    map_res(named_weekday, |weekday: Weekday| {
        Ok(naive_date_for_weekday(weekday))
    })(input)
}

/// Recognizes the `case insensitive` weekday in `Russian` using the
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
/// use nom_date_parsers::{i18n::ru::current_named_weekday_only_opt, numeric::dd_only};
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
/// `language-specific` dates in `Russian`:
/// - Numeric date parsers:
///     - [`dd_only`]
///     - [`dd_mm_only`]
///     - [`dd_mm_y4`]
/// - Language-specific
///     - [`day_before_yesterday_opt`]
///     - [`yesterday_opt`]
///     - [`tomorrow_opt`]
///     - [`day_after_tomorrow_opt`]
///     - [`current_named_weekday_only_opt`]
pub fn bundle(input: &str) -> IResult<&str, Option<NaiveDate>> {
    alt((
        dd_mm_y4,
        dd_mm_only,
        dd_only,
        day_before_yesterday_opt,
        yesterday_opt,
        tomorrow_opt,
        day_after_tomorrow_opt,
        current_named_weekday_only_opt,
    ))(input)
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Local};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("позавчера", Ok(("", Local::now().sub(Days::new(2)).date_naive())))]
    fn test_day_before_yesterday(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(day_before_yesterday(input), expected);
    }

    #[rstest]
    #[case("позавчера", Ok(("", Some(Local::now().sub(Days::new(2)).date_naive()))))]
    fn test_day_before_yesterday_opt(
        #[case] input: &str,
        #[case] expected: IResult<&str, Option<NaiveDate>>,
    ) {
        assert_eq!(day_before_yesterday_opt(input), expected);
    }

    #[rstest]
    #[case("Вчера", Ok(("", Local::now().sub(Days::new(1)).date_naive())))]
    fn test_yesterday(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(yesterday(input), expected);
    }

    #[rstest]
    #[case("Вчера", Ok(("", Some(Local::now().sub(Days::new(1)).date_naive()))))]
    fn test_yesterday_opt(#[case] input: &str, #[case] expected: IResult<&str, Option<NaiveDate>>) {
        assert_eq!(yesterday_opt(input), expected)
    }

    #[rstest]
    #[case("Завтра", Ok(("", Local::now().add(Days::new(1)).date_naive())))]
    fn test_tomorrow(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(tomorrow(input), expected);
    }

    #[rstest]
    #[case("Завтра", Ok(("", Some(Local::now().add(Days::new(1)).date_naive()))))]
    fn test_tomorrow_opt(#[case] input: &str, #[case] expected: IResult<&str, Option<NaiveDate>>) {
        assert_eq!(tomorrow_opt(input), expected);
    }

    #[rstest]
    #[case("Послезавтра", Ok(("", Local::now().add(Days::new(2)).date_naive())))]
    fn test_day_after_tomorrow(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(day_after_tomorrow(input), expected);
    }

    #[rstest]
    #[case("послезавтра", Ok(("", Some(Local::now().add(Days::new(2)).date_naive()))))]
    fn test_day_after_tomorrow_opt(
        #[case] input: &str,
        #[case] expected: IResult<&str, Option<NaiveDate>>,
    ) {
        assert_eq!(day_after_tomorrow_opt(input), expected);
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

    #[rstest]
    #[case("пн", Ok(("", Some(naive_date_for_weekday(Weekday::Mon)))))]
    #[case("Вторник", Ok(("", Some(naive_date_for_weekday(Weekday::Tue)))))]
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
    #[case("позавчера", Ok(("", Some(Local::now().sub(Days::new(2)).date_naive()))))]
    #[case("Вчера", Ok(("", Some(Local::now().sub(Days::new(1)).date_naive()))))]
    #[case("Завтра", Ok(("", Some(Local::now().add(Days::new(1)).date_naive()))))]
    #[case("послезавтра", Ok(("", Some(Local::now().add(Days::new(2)).date_naive()))))]
    fn test_bundle(#[case] input: &str, #[case] expected: IResult<&str, Option<NaiveDate>>) {
        assert_eq!(bundle(input), expected)
    }
}
