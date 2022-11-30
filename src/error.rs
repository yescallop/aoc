use std::{io, num::ParseIntError, panic::Location, string::FromUtf8Error};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("curl: {0}")]
    Curl(#[from] curl::Error),
    #[error("solution not found")]
    SolutionNotFound,
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Error {
        Error::Io(io::Error::new(
            io::ErrorKind::InvalidData,
            "stream did not contain valid UTF-8",
        ))
    }
}

pub trait Track<T> {
    fn ok(self) -> Result<T, Error>;
}

impl<T> Track<T> for Option<T> {
    #[track_caller]
    fn ok(self) -> Result<T, Error> {
        let location = Location::caller();
        self.ok_or_else(|| {
            Error::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "failure at {}:{}:{}",
                    location.file(),
                    location.line(),
                    location.column()
                ),
            ))
        })
    }
}

macro_rules! impl_error_from {
    ($($ty:ty),+) => {
        $(impl From<$ty> for Error {
            #[track_caller]
            fn from(e: $ty) -> Error {
                let location = Location::caller();
                Error::Io(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "{}, at {}:{}:{}",
                        e,
                        location.file(),
                        location.line(),
                        location.column()
                    ),
                ))
            }
        })+
    };
}

impl_error_from!(ParseIntError);
