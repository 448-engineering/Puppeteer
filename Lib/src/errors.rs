use std::io::ErrorKind;
use tao::{
    error::{ExternalError, OsError},
    event_loop::EventLoopClosed,
};
use thiserror::Error;
use wry::Error as WryError;

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
    TaoEventLoopClosed,
    /// An error whose cause it outside the control of the running app
    #[error("An error whose cause it outside the control of the running app")]
    TaoExternal(String),
    /// The error type for when the OS cannot perform the requested operation
    #[error("The error type for when the OS cannot perform the requested operation")]
    TaoOsError(String),
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
    /// An [std::io::ErrorKind]
    #[error("Represents std::io::ErrorKind")]
    Io(ErrorKind),
    /// For webview only WOFF2 font format is supported.
    #[error("The font detected ({0:?}) is not a valid `WOFF2` format for the web.")]
    InvalidFontExpectedWoff2(String),
    /// Tried to get a file name without the extension part using `Path::file_stem()` but the file stem does not exist
    #[error("Tried to get a file name without the extension part using `Path::file_stem()` but the file stem does not exist")]
    InvalidFileStemName,
    /// The file being read has exceeded the maximum file size set
    #[error("The maximum size set for the resource has been exceeded")]
    MaxResourceLengthExceeded,
    /// Encountered a GTK error on Linux
    #[error("Encountered a GTK error on Linux")]
    GtkError,
}

impl From<std::io::Error> for PuppeteerError {
    fn from(error: std::io::Error) -> Self {
        PuppeteerError::Io(error.kind())
    }
}

impl<T> From<EventLoopClosed<T>> for PuppeteerError {
    fn from(_: EventLoopClosed<T>) -> Self {
        PuppeteerError::TaoEventLoopClosed
    }
}

impl From<ExternalError> for PuppeteerError {
    fn from(value: ExternalError) -> Self {
        PuppeteerError::TaoExternal(value.to_string())
    }
}

impl From<WryError> for PuppeteerError {
    fn from(value: WryError) -> Self {
        PuppeteerError::Wry(value)
    }
}

impl From<OsError> for PuppeteerError {
    fn from(value: OsError) -> Self {
        PuppeteerError::TaoOsError(value.to_string())
    }
}
