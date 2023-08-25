use thiserror::Error;
use wry::{
    application::{
        error::{ExternalError, OsError},
        event_loop::EventLoopClosed,
    },
    Error as WryError,
};

/// A wrapper to `Result<T, PuppeteerError>`
pub type PuppeteerResult<T> = Result<T, PuppeteerError>;

/// The error type for Puppeteer
#[derive(Debug, Error)]
pub enum PuppeteerError {
    /// An error occured while initializing the app
    #[error("An error occurred while initializing the app")]
    AppInit(String),
    /// The error that is returned when an EventLoopProxy attempts to wake up an EventLoop that no longer exists. Contains the original event given to send_event.
    #[error("The error that is returned when an EventLoopProxy attempts to wake up an EventLoop that no longer exists. Contains the original event given to send_event.
    ")]
    WryEventLoopClosed,
    /// An error whose cause it outside the control of the running app
    #[error("An error whose cause it outside the control of the running app")]
    WryExternal(String),
    /// The error type for when the OS cannot perform the requested operation
    #[error("The error type for when the OS cannot perform the requested operation")]
    WryOsError(String),
    /// Errors returned by wry
    #[error("Errors returned by wry")]
    Wry(WryError),
    /// The app is unable to determine the primary monitor
    #[error("Unable to detect current monitor")]
    UnableToDetectCurrentMonitor,
    /// The app is unable to detect primary monitor
    #[error("Unable to detect primary monitor")]
    UnableToDetectPrimaryMonitor,
    /// The webview does not exist. It might have been terminated
    #[error("The `WebView` was not found")]
    WebViewDoesNotExist,
    /// The window is not resizable therefore the resize operation is impossible
    #[error("The `window is not resizable")]
    WindowIsNotResizable,
}

impl<T> From<EventLoopClosed<T>> for PuppeteerError {
    fn from(_: EventLoopClosed<T>) -> Self {
        PuppeteerError::WryEventLoopClosed
    }
}

impl From<ExternalError> for PuppeteerError {
    fn from(value: ExternalError) -> Self {
        PuppeteerError::WryExternal(value.to_string())
    }
}

impl From<WryError> for PuppeteerError {
    fn from(value: WryError) -> Self {
        PuppeteerError::Wry(value)
    }
}

impl From<OsError> for PuppeteerError {
    fn from(value: OsError) -> Self {
        PuppeteerError::WryOsError(value.to_string())
    }
}
