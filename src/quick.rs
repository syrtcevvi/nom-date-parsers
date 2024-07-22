use std::ops::{Add, Sub};

use chrono::{Days, Local, NaiveDate};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::map_res,
    sequence::tuple,
};

use crate::types::IResult;

/// Recognizes the `+ <u64>` pattern, where the `<u64>` is an unsigned 64-bit
/// integer and returns the `Option<NaiveDate>` which is obtained by adding
/// specified number of days to today.
///
/// # Examples
/// ```
/// use std::ops::Add;
///
/// use chrono::{Days, Local};
/// use nom_date_parsers::quick::forward_from_now_opt;
///
/// assert_eq!(
///     forward_from_now_opt("+ 42")?.1,
///     Some(Local::now().add(Days::new(42)).date_naive())
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn forward_from_now_opt(input: &str) -> IResult<&str, Option<NaiveDate>> {
    let (input, (_, _, add_days)) = tuple((
        tag("+"),
        space0,
        map_res(digit1, |s: &str| s.parse::<u64>()),
    ))(input)?;

    Ok((
        input,
        Some(Local::now().add(Days::new(add_days)).date_naive()),
    ))
}

/// Recognizes the `- <u64>` pattern, where the `<u64>` is an unsigned 64-bit
/// integer, and returns the `Option<NaiveDate>` which is obtained by
/// subtraction specified number of days from today.
///
/// # Examples
/// ```
/// use std::ops::Sub;
///
/// use chrono::{Local, Days};
/// use nom_date_parsers::quick::backward_from_now_opt;
///
/// assert_eq!(backward_from_now_opt("- 42")?.1, Some(Local::now().sub(Days::new(42)).date_naive()));
/// # Ok::<(), Box<dyn std::error::Error>>(())
pub fn backward_from_now_opt(input: &str) -> IResult<&str, Option<NaiveDate>> {
    let (input, (_, _, sub_days)) = tuple((
        tag("-"),
        space0,
        map_res(digit1, |s: &str| s.parse::<u64>()),
    ))(input)?;

    Ok((
        input,
        Some(Local::now().sub(Days::new(sub_days)).date_naive()),
    ))
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("+ 1", Ok(("", Some(Local::now().add(Days::new(1)).date_naive()))))]
    #[case("+42", Ok(("", Some(Local::now().add(Days::new(42)).date_naive()))))]
    fn test_forward_from_now_opt_test(
        #[case] input: &str,
        #[case] expected: IResult<&str, Option<NaiveDate>>,
    ) {
        assert_eq!(forward_from_now_opt(input), expected);
    }

    #[rstest]
    #[case("- 1", Ok(("", Some(Local::now().sub(Days::new(1)).date_naive()))))]
    #[case("-123", Ok(("", Some(Local::now().sub(Days::new(123)).date_naive()))))]
    fn test_backward_from_now_opt_test(
        #[case] input: &str,
        #[case] expected: IResult<&str, Option<NaiveDate>>,
    ) {
        assert_eq!(backward_from_now_opt(input), expected);
    }
}
