use wry::{
    application::{
        error::{ExternalError, OsError},
        event_loop::EventLoopClosed,
    },
    Error as WryError,
};

pub type PuppeteerResult<T> = Result<T, PuppeteerError<T>>;

#[derive(Debug)]
pub enum PuppeteerError<T: 'static> {
    WryEventLoopClosed(T),
    WryExternal(String),
    WryOsError(String),
    Wry(WryError),
}

impl<T> From<EventLoopClosed<T>> for PuppeteerError<T> {
    fn from(value: EventLoopClosed<T>) -> Self {
        PuppeteerError::WryEventLoopClosed(value.0)
    }
}

impl<T> From<ExternalError> for PuppeteerError<T> {
    fn from(value: ExternalError) -> Self {
        PuppeteerError::WryExternal(value.to_string())
    }
}

impl<T> From<WryError> for PuppeteerError<T> {
    fn from(value: WryError) -> Self {
        PuppeteerError::Wry(value)
    }
}

impl<T> From<OsError> for PuppeteerError<T> {
    fn from(value: OsError) -> Self {
        PuppeteerError::WryOsError(value.to_string())
    }
}
