use crate::error::Error;

pub type IResult<I, O> = std::result::Result<(I, O), nom::Err<Error<I>>>;
