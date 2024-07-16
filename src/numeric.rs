use chrono::{Datelike, Local, NaiveDate};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::space1,
    combinator::map_res,
    sequence::{separated_pair, tuple},
};

use crate::{error::Error, types::IResult};

/// Recognizes a separator of numeric date parts in the following templates (asterisk symbol denotes some separator):
/// - dd*mm*yyyy
/// - mm*dd*yyyy
/// - yyyy*mm*dd
pub fn numeric_date_parts_separator(input: &str) -> IResult<&str, ()> {
    let (input, _) = alt((tag("/"), tag("-"), tag("."), space1))(input)?;

    Ok((input, ()))
}

/// Recognizes either one or two digits of a `day` part
///
/// Accepts numbers in the range `01..=31`, otherwise returns [`Error::DayOutOfRange`]
///
/// It can be used to recognize the `dd` part in the `dd`/mm/yyyy pattern, for instance
pub fn dd(input: &str) -> IResult<&str, u32> {
    let (input, dd) = alt((
        map_res(take(2_u8), |s: &str| s.parse()),
        map_res(take(1_u8), |s: &str| s.parse()),
    ))(input)?;

    if dd == 0 || dd > 31 {
        return Err(nom::Err::Error(Error::DayOutOfRange));
    }
    Ok((input, dd))
}

/// Recognizes either one or two digits of a `day` part and returns the [`NaiveDate`] with the selected
/// day and current month and year if the date exists, otherwise returns `None`
///
/// ```
/// use chrono::prelude::*;
/// use nom_date_parsers::prelude::*;
///
/// assert_eq!(dd_only("13")?.1, Local::now().date_naive().with_day(13));
/// assert_eq!(dd_only("42"), Err(nom::Err::Error(Error::DayOutOfRange)));
///
/// # Ok::<(), Box::<dyn std::error::Error>>(())
/// ```
pub fn dd_only(input: &str) -> IResult<&str, Option<NaiveDate>> {
    let (input, day) = dd(input)?;
    let now = Local::now();
    let (month, year) = (now.month(), now.year());

    Ok((input, NaiveDate::from_ymd_opt(year, month, day)))
}

/// Recognizes either one or two digits of a `month` part
///
/// Accepts numbers in the range `01..=12`, otherwise returns [`Error::MonthOutOfRange`]
pub fn mm(input: &str) -> IResult<&str, u32> {
    let (input, mm) = alt((
        map_res(take(2_u8), |s: &str| s.parse()),
        map_res(take(1_u8), |s: &str| s.parse()),
    ))(input)?;
    if mm == 0 || mm > 12 {
        return Err(nom::Err::Error(Error::MonthOutOfRange));
    }

    Ok((input, mm))
}

/// Recognizes the `day` and `month` parts separated by the [`numeric_date_parts_separator`] using the [`dd`] and [`mm`] parsers
pub fn dd_mm(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(dd, numeric_date_parts_separator, mm)(input)
}

/// Recognizes the `day` and `month` parts separated by the [`numeric_date_parts_separator`] and returns the [`NaiveDate`] with the selected
/// day, month and current year if the date exists, otherwise returns `None`
///
/// ```
/// use chrono::prelude::*;
/// use nom_date_parsers::prelude::*;
///
/// assert_eq!(dd_mm_only("18-10")?.1, NaiveDate::from_ymd_opt(Local::now().year(), 10, 18));
///
/// # Ok::<(), Box::<dyn std::error::Error>>(())
/// ```
pub fn dd_mm_only(input: &str) -> IResult<&str, Option<NaiveDate>> {
    let (input, (day, month)) = dd_mm(input)?;
    let year = Local::now().year();

    Ok((input, NaiveDate::from_ymd_opt(year, month, day)))
}

/// Recognizes the `month` and `day` parts separated by the [`numeric_date_parts_separator`] using the [`mm`] and [`dd`] parsers
pub fn mm_dd(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(mm, numeric_date_parts_separator, dd)(input)
}

/// Recognizes the `month` and `day` parts separated by the [`numeric_date_parts_separator`] and returns the [`NaiveDate`] with the selected
/// day, month and current year if the date exists, otherwise returns `None`
///
/// ```
/// use chrono::prelude::*;
/// use nom_date_parsers::prelude::*;
///
/// assert_eq!(mm_dd_only("10/18")?.1, NaiveDate::from_ymd_opt(Local::now().year(), 10, 18));
///
/// # Ok::<(), Box::<dyn std::error::Error>>(())
/// ```
pub fn mm_dd_only(input: &str) -> IResult<&str, Option<NaiveDate>> {
    let (input, (month, day)) = mm_dd(input)?;

    Ok((
        input,
        NaiveDate::from_ymd_opt(Local::now().year(), month, day),
    ))
}

/// Recognizes four digits of the `year` part
///
/// Accepts numbers in the range `0000..=9999`, technically
pub fn y4(input: &str) -> IResult<&str, u32> {
    map_res(take(4_u8), |s: &str| s.parse::<u32>())(input)
}

/// Recognizes the `year`, `month` and `day` parts separated by the [`numeric_date_parts_separator`] and returns [`NaiveDate`] with the selected parts
/// if the date exists, otherwise returns `None`
///
/// ```
/// use chrono::prelude::*;
/// use nom_date_parsers::prelude::*;
///
/// assert_eq!(y4_mm_dd("2024-07-13")?.1, NaiveDate::from_ymd_opt(2024, 7, 13));
///
/// # Ok::<(), Box::<dyn std::error::Error>>(())
/// ```
pub fn y4_mm_dd(input: &str) -> IResult<&str, Option<NaiveDate>> {
    let (input, (y4, (), mm, (), dd)) = tuple((
        y4,
        numeric_date_parts_separator,
        mm,
        numeric_date_parts_separator,
        dd,
    ))(input)?;

    Ok((input, NaiveDate::from_ymd_opt(y4 as i32, mm, dd)))
}

/// Recognizes the `day`, `month` and `year` parts separated by the [`numeric_date_parts_separator`] and returns [`NaiveDate`] with the selected parts
/// if the date exists, otherwise returns `None`
///
/// ```
/// use chrono::prelude::*;
/// use nom_date_parsers::prelude::*;
///
/// assert_eq!(dd_mm_y4("13/07/2024")?.1, NaiveDate::from_ymd_opt(2024, 7, 13));
///
/// # Ok::<(), Box::<dyn std::error::Error>>(())
/// ```
pub fn dd_mm_y4(input: &str) -> IResult<&str, Option<NaiveDate>> {
    let (input, (dd, (), mm, (), y4)) = tuple((
        dd,
        numeric_date_parts_separator,
        mm,
        numeric_date_parts_separator,
        y4,
    ))(input)?;

    Ok((input, NaiveDate::from_ymd_opt(y4 as i32, mm, dd)))
}

/// Recognizes the `month`, `day` and `year` parts separated by the [`numeric_date_parts_separator`] and returns [`NaiveDate`] with the selected parts
/// if the date exists, otherwise returns `None`
///
/// ```
/// use chrono::prelude::*;
/// use nom_date_parsers::prelude::*;
///
/// assert_eq!(mm_dd_y4("07-13-2024")?.1, NaiveDate::from_ymd_opt(2024, 7, 13));
///
/// # Ok::<(), Box::<dyn std::error::Error>>(())
/// ```
pub fn mm_dd_y4(input: &str) -> IResult<&str, Option<NaiveDate>> {
    let (input, (mm, (), dd, (), y4)) = tuple((
        mm,
        numeric_date_parts_separator,
        dd,
        numeric_date_parts_separator,
        y4,
    ))(input)?;

    Ok((input, NaiveDate::from_ymd_opt(y4 as i32, mm, dd)))
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use nom::error::ErrorKind;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    fn now_date_naive() -> NaiveDate {
        Local::now().date_naive()
    }

    #[rstest]
    #[case("9", Ok(("", 9)))]
    #[case("09", Ok(("", 9)))]
    #[case("31", Ok(("", 31)))]
    #[case("00", Err(nom::Err::Error(Error::DayOutOfRange)))]
    #[case("42", Err(nom::Err::Error(Error::DayOutOfRange)))]
    fn test_dd(#[case] input: &str, #[case] expected: IResult<&str, u32>) {
        assert_eq!(dd(input), expected);
    }

    #[rstest]
    #[case("9", Ok(("", now_date_naive().with_day(9))))]
    #[case("09", Ok(("", now_date_naive().with_day(9))))]
    #[case("31", Ok(("", now_date_naive().with_day(31))))]
    #[case("00", Err(nom::Err::Error(Error::DayOutOfRange)))]
    #[case("42", Err(nom::Err::Error(Error::DayOutOfRange)))]
    fn test_dd_only(#[case] input: &str, #[case] expected: IResult<&str, Option<NaiveDate>>) {
        assert_eq!(dd_only(input), expected)
    }

    #[rstest]
    #[case("9", Ok(("", 9)))]
    #[case("09", Ok(("", 9)))]
    #[case("12", Ok(("", 12)))]
    #[case("00", Err(nom::Err::Error(Error::MonthOutOfRange)))]
    #[case("13", Err(nom::Err::Error(Error::MonthOutOfRange)))]
    fn test_mm(#[case] input: &str, #[case] expected: IResult<&str, u32>) {
        assert_eq!(mm(input), expected);
    }

    #[rstest]
    #[case("3/9", Ok(("", now_date_naive().with_day(3).unwrap().with_month(9))))]
    #[case("03-09", Ok(("", now_date_naive().with_day(3).unwrap().with_month(9))))]
    #[case("03/12", Ok(("", now_date_naive().with_day(3).unwrap().with_month(12))))]
    #[case("00", Err(nom::Err::Error(Error::DayOutOfRange)))]
    #[case("42", Err(nom::Err::Error(Error::DayOutOfRange)))]
    #[case("13.00", Err(nom::Err::Error(Error::MonthOutOfRange)))]
    #[case("13\t13", Err(nom::Err::Error(Error::MonthOutOfRange)))]
    fn test_dd_mm_only(#[case] input: &str, #[case] expected: IResult<&str, Option<NaiveDate>>) {
        assert_eq!(dd_mm_only(input), expected);
    }

    #[rstest]
    #[case("0042", Ok(("", 42)))]
    #[case("2024", Ok(("", 2024)))]
    #[case("42", Err(nom::Err::Error(Error::Nom("42", ErrorKind::Eof))))]
    #[case("10001", Ok(("1", 1000)))]
    fn test_y4(#[case] input: &str, #[case] expected: IResult<&str, u32>) {
        assert_eq!(y4(input), expected);
    }

    #[rstest]
    #[case("2024-06-13", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13))))]
    #[case("2024/06-13", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13))))]
    #[case("2024.06.13", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13))))]
    #[case("2024    06\t13", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13))))]
    #[case("2024/00/06", Err(nom::Err::Error(Error::MonthOutOfRange)))]
    #[case("2024/13/06", Err(nom::Err::Error(Error::MonthOutOfRange)))]
    #[case("2024/10/00", Err(nom::Err::Error(Error::DayOutOfRange)))]
    #[case("2024/10/42", Err(nom::Err::Error(Error::DayOutOfRange)))]
    fn test_y4_mm_dd(#[case] input: &str, #[case] expected: IResult<&str, Option<NaiveDate>>) {
        assert_eq!(y4_mm_dd(input), expected);
    }

    #[rstest]
    #[case("13-06-2024", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13))))]
    #[case("13/06-2024", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13))))]
    #[case("13.06.2024", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13))))]
    #[case("13    06\t2024", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13))))]
    #[case("00/10/2024", Err(nom::Err::Error(Error::DayOutOfRange)))]
    #[case("42/10/2024", Err(nom::Err::Error(Error::DayOutOfRange)))]
    #[case("06/00/2024", Err(nom::Err::Error(Error::MonthOutOfRange)))]
    #[case("06/13/2024", Err(nom::Err::Error(Error::MonthOutOfRange)))]
    fn test_dd_mm_y4(#[case] input: &str, #[case] expected: IResult<&str, Option<NaiveDate>>) {
        assert_eq!(dd_mm_y4(input), expected);
    }

    #[rstest]
    #[case("06-13-2024", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13))))]
    #[case("06/13-2024", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13))))]
    #[case("06.13.2024", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13))))]
    #[case("06    13\t2024", Ok(("", NaiveDate::from_ymd_opt(2024, 6, 13))))]
    #[case("00/06/2024", Err(nom::Err::Error(Error::MonthOutOfRange)))]
    #[case("13/06/2024", Err(nom::Err::Error(Error::MonthOutOfRange)))]
    #[case("10/00/2024", Err(nom::Err::Error(Error::DayOutOfRange)))]
    #[case("10/32/2024", Err(nom::Err::Error(Error::DayOutOfRange)))]
    fn test_mm_dd_y4(#[case] input: &str, #[case] expected: IResult<&str, Option<NaiveDate>>) {
        assert_eq!(mm_dd_y4(input), expected)
    }
}
