#[cfg(feature = "native")]
use wry::{
    application::{
        error::{ExternalError, OsError},
        event_loop::EventLoopClosed,
    },
    Error as WryError,
};

#[cfg(not(feature = "native"))]
pub type PuppeteerResult<T> = Result<T, PuppeteerError>;

#[cfg(feature = "native")]
pub type PuppeteerResult<T> = Result<T, PuppeteerError<T>>;

#[derive(Debug)]
pub enum PuppeteerError<#[cfg(feature = "native")] T: 'static> {
    #[cfg(feature = "native")]
    WryEventLoopClosed(T),
    #[cfg(feature = "native")]
    WryExternal(String),
    #[cfg(feature = "native")]
    WryOsError(String),
    #[cfg(feature = "native")]
    Wry(WryError),
}

#[cfg(feature = "native")]
impl<T> From<EventLoopClosed<T>> for PuppeteerError<T> {
    fn from(value: EventLoopClosed<T>) -> Self {
        PuppeteerError::WryEventLoopClosed(value.0)
    }
}

#[cfg(feature = "native")]
impl<T> From<ExternalError> for PuppeteerError<T> {
    fn from(value: ExternalError) -> Self {
        PuppeteerError::WryExternal(value.to_string())
    }
}

#[cfg(feature = "native")]
impl<T> From<WryError> for PuppeteerError<T> {
    fn from(value: WryError) -> Self {
        PuppeteerError::Wry(value)
    }
}

#[cfg(feature = "native")]
impl<T> From<OsError> for PuppeteerError<T> {
    fn from(value: OsError) -> Self {
        PuppeteerError::WryOsError(value.to_string())
    }
}
