use std::ops::{Add, Sub};

use chrono::{Days, Local, NaiveDate};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::map_res,
    sequence::tuple,
};

use crate::types::IResult;

/// Recognizes the `+ <u64>` pattern, where the `<u64>` is an unsigned 64-bit
/// integer and returns the `NaiveDate` which is obtained by adding
/// specified number of days to today.
///
/// # Examples
/// ```
/// use std::ops::Add;
///
/// use chrono::{Days, Local};
/// use nom_date_parsers::quick::forward_from_now;
///
/// assert_eq!(
///     forward_from_now("+ 42")?.1,
///     Local::now().add(Days::new(42)).date_naive()
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn forward_from_now(input: &str) -> IResult<&str, NaiveDate> {
    let (input, (_, _, add_days)) = tuple((
        tag("+"),
        space0,
        map_res(digit1, |s: &str| s.parse::<u64>()),
    ))(input)?;

    Ok((input, Local::now().add(Days::new(add_days)).date_naive()))
}

/// Recognizes the `- <u64>` pattern, where the `<u64>` is an unsigned 64-bit
/// integer, and returns the `NaiveDate` which is obtained by
/// subtraction specified number of days from today.
///
/// # Examples
/// ```
/// use std::ops::Sub;
///
/// use chrono::{Local, Days};
/// use nom_date_parsers::quick::backward_from_now;
///
/// assert_eq!(backward_from_now("- 42")?.1, Local::now().sub(Days::new(42)).date_naive());
/// # Ok::<(), Box<dyn std::error::Error>>(())
pub fn backward_from_now(input: &str) -> IResult<&str, NaiveDate> {
    let (input, (_, _, sub_days)) = tuple((
        tag("-"),
        space0,
        map_res(digit1, |s: &str| s.parse::<u64>()),
    ))(input)?;

    Ok((input, Local::now().sub(Days::new(sub_days)).date_naive()))
}

/// Parser that uses the [`backward_from_now`] and [`forward_from_now`]
/// parsers to recognize the following patterns: `- <nod>` and `+ <nod>` (`nod`
/// - number of days)
pub fn bundle(input: &str) -> IResult<&str, NaiveDate> {
    alt((forward_from_now, backward_from_now))(input)
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("+ 1", Ok(("", Local::now().add(Days::new(1)).date_naive())))]
    #[case("+42", Ok(("", Local::now().add(Days::new(42)).date_naive())))]
    fn test_forward_from_now_opt_test(
        #[case] input: &str,
        #[case] expected: IResult<&str, NaiveDate>,
    ) {
        assert_eq!(forward_from_now(input), expected);
    }

    #[rstest]
    #[case("- 1", Ok(("", Local::now().sub(Days::new(1)).date_naive())))]
    #[case("-123", Ok(("", Local::now().sub(Days::new(123)).date_naive())))]
    fn test_backward_from_now_opt_test(
        #[case] input: &str,
        #[case] expected: IResult<&str, NaiveDate>,
    ) {
        assert_eq!(backward_from_now(input), expected);
    }

    #[rstest]
    #[case("-   1", Ok(("", Local::now().sub(Days::new(1)).date_naive())))]
    #[case("-123", Ok(("", Local::now().sub(Days::new(123)).date_naive())))]
    #[case("+\t42", Ok(("", Local::now().add(Days::new(42)).date_naive())))]
    fn test_bundle(#[case] input: &str, #[case] expected: IResult<&str, NaiveDate>) {
        assert_eq!(bundle(input), expected);
    }
}
