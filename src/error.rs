use std::num::ParseIntError;

use nom::error::{ErrorKind, FromExternalError, ParseError};

#[derive(Debug, PartialEq)]
pub enum Error<I> {
    DayOutOfRange,
    MonthOutOfRange,
    NonExistentDate,
    ParseIntError(I, ErrorKind, ParseIntError),

    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        Error::Nom(input, kind)
    }

    fn append(_input: I, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I> FromExternalError<I, ParseIntError> for Error<I> {
    fn from_external_error(input: I, kind: ErrorKind, e: ParseIntError) -> Self {
        Self::ParseIntError(input, kind, e)
    }
}
