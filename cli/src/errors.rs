use std::io::{Error as StdIoError, ErrorKind};

/// Represents the Result with generic value `T` and an `InjectorError`
pub type InjectorResult<T> = Result<T, InjectorError>;

#[derive(Debug)]
pub enum InjectorError {
    Io(ErrorKind),
}

impl From<StdIoError> for InjectorError {
    fn from(value: StdIoError) -> Self {
        InjectorError::Io(value.kind())
    }
}
