use std::{io, num::ParseIntError, string::FromUtf8Error};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("curl: {0}")]
    Curl(#[from] curl::Error),
    #[error("solution not found")]
    SolutionNotFound,
}

macro_rules! impl_error_from {
    ($($ty:ty),+) => {
        $(impl From<$ty> for Error {
            fn from(e: $ty) -> Error {
                Error::Io(io::Error::new(io::ErrorKind::InvalidData, e))
            }
        })+
    };
}

impl_error_from!(&str, FromUtf8Error, ParseIntError);
