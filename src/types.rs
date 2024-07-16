use crate::error::Error;

pub(crate) type IResult<I, O> = std::result::Result<(I, O), nom::Err<Error<I>>>;
