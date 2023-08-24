use thiserror::Error;
use wry::{
    application::{
        error::{ExternalError, OsError},
        event_loop::EventLoopClosed,
    },
    Error as WryError,
};

/// A wrapper to `Result<T, PuppeteerError>`
pub type PuppeteerResult<T, U> = Result<T, PuppeteerError<U>>;

/// The error type for Puppeteer
#[derive(Debug, Error)]
pub enum PuppeteerError<AppTrait> {
    /// An error occured while initializing the app
    #[error("An error occurred while initializing the app")]
    AppInit(String),
    /// The error that is returned when an EventLoopProxy attempts to wake up an EventLoop that no longer exists. Contains the original event given to send_event.
    #[error("The error that is returned when an EventLoopProxy attempts to wake up an EventLoop that no longer exists. Contains the original event given to send_event.
    ")]
    WryEventLoopClosed(AppTrait),
    /// An error whose cause it outside the control of the running app
    #[error("An error whose cause it outside the control of the running app")]
    WryExternal(String),
    /// The error type for when the OS cannot perform the requested operation
    #[error("The error type for when the OS cannot perform the requested operation")]
    WryOsError(String),
    /// Errors returned by wry
    #[error("Errors returned by wry")]
    Wry(WryError),
}

impl<U> From<EventLoopClosed<U>> for PuppeteerError<U> {
    fn from(value: EventLoopClosed<U>) -> Self {
        PuppeteerError::WryEventLoopClosed(value.0)
    }
}

impl<U> From<ExternalError> for PuppeteerError<U> {
    fn from(value: ExternalError) -> Self {
        PuppeteerError::WryExternal(value.to_string())
    }
}

impl<U> From<WryError> for PuppeteerError<U> {
    fn from(value: WryError) -> Self {
        PuppeteerError::Wry(value)
    }
}

impl<U> From<OsError> for PuppeteerError<U> {
    fn from(value: OsError) -> Self {
        PuppeteerError::WryOsError(value.to_string())
    }
}
